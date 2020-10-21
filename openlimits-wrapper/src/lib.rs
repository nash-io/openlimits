use openlimits::{exchange::Exchange, exchange_ws::ExchangeWs, nash::Nash, model::{OrderBookRequest, OrderBookResponse, websocket::{OpenLimitsWebsocketMessage, Subscription}}};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::stream::StreamExt;
use tokio::sync::Mutex;
use std::net::Shutdown;
use std::error::Error;
use std::sync::Arc;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OpenlimitsRequest {
    // TODO: add others
    Orderbook(OrderBookRequest)
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OpenlimitsResponse {
    // TODO: add others
    Orderbook(OrderBookResponse),
    Error(String)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SubscriptionMetaRequest {
    Subscribe
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SubscriptionMetaResponse {
    SubscriptionStarted,
    Error(String)
}

/// Representation for core data for openlimits server process
pub struct OpenlimitsServer {
    pub exchange: Arc<Mutex<Nash>>, // generalize this to exchange in the future
    pub port: String
}

impl OpenlimitsServer {
    /// Start server loop, listening to and responding to connections
    pub async fn start_socket_server(&self) -> Result<(), Box<dyn Error + Send + Sync>>{
        let mut listener = TcpListener::bind(&format!("127.0.0.1:{}", self.port)).await?;
        let mut incoming = listener.incoming();

        while let Some(stream) = incoming.next().await {
            let mut stream = stream?;
            let exchange = self.exchange.clone();
            // each connection goes into its own async thread
            tokio::spawn(async move {
                let mut request = String::new();
                let _num_bytes_read = stream.read_to_string(&mut request).await.expect("failed to read");
                // This is a request to read data from subscripion stream. Keep connection open and push back all data
                if let Ok(_) = serde_json::from_str::<SubscriptionMetaRequest>(&request) {
                    // Here we feed back all subscription data over the connection
                    // First one byte with length (in bytes), then the payload
                }
                // This is a request to execute a new subscription. Data will be returned on the global subscrition stream
                // which should be requested independently. Send back confirmation then client closes connection
                else if let Ok(ws_request) = serde_json::from_str::<Subscription>(&request) {
                    let response = match process_incoming_ws_request(ws_request, exchange).await {
                        // Subscription was started successfully with openlimits
                        Ok(_) => serde_json::to_string(&SubscriptionMetaResponse::SubscriptionStarted).unwrap(),
                        // Something went wrong
                        Err(e) => serde_json::to_string(&SubscriptionMetaResponse::Error(format!("{:?}",e))).unwrap(),
                    };
                    stream.write(response.as_bytes()).await.expect("failed to write response");
                }
                // This is a direct API request. Response will be returned directly then connection closed by client
                else if let Ok(openlimits_request) = serde_json::from_str::<OpenlimitsRequest>(&request) {
                    match process_incoming_request(openlimits_request, exchange).await {
                        // Return response from openlimits
                        Ok(response) => {
                            stream.write(response.as_bytes()).await.expect("failed to write response");
                        }
                        // Something went wrong
                        Err(e) => {
                            let err = SubscriptionMetaResponse::Error(format!("{:?}",e));
                            let err_response = serde_json::to_string(&err).unwrap();
                            stream.write(err_response.as_bytes()).await.expect("failed to write response");
                        }
                    } 
                }
                // Any other sort of request is invalid
                else {
                    stream.shutdown(Shutdown::Both).unwrap();
                }
                
            });
        }
        Ok(())

    }
}

async fn process_incoming_ws_request(request: Subscription, exchange: Arc<Mutex<Nash>>) -> Result<(), Box<dyn Error + Send + Sync>>{
    // match on enum and thread through to appropriate Exchange trait method, then start subscription
    // subscription data will be retrieved on another connection 
    let mut exchange = exchange.lock().await;
    exchange.subscribe(request).await?;
    Ok(())
}

async fn process_incoming_request(request: OpenlimitsRequest, exchange: Arc<Mutex<Nash>>) -> Result<String, Box<dyn Error + Send + Sync>> {
    // match on enum and thread through to appropriate Exchange trait method, then return result
    let exchange = exchange.lock().await;
    let response = match request {
        OpenlimitsRequest::Orderbook(req) => {
            serde_json::to_string(&exchange.order_book(&req).await?).map_err(|x| x.into())
        },
        _ => Err("request not supported".into())
    };
    response
}

#[cfg(test)]
mod tests {
    use super::SubscriptionMetaRequest;
    #[test]
    fn serialize_request_for_subscription_stream() {
        let req = SubscriptionMetaRequest::Subscribe;
        let sub_req = serde_json::to_string(&req).unwrap();
        println!("{:?}", sub_req);
    }
}
