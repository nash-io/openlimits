use openlimits::{exchange::Exchange, nash::Nash, model::{OrderBookRequest, OrderBookResponse, websocket::{OpenLimitsWebsocketMessage, Subscription}}};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::prelude::*;
use std::error::Error;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OpenlimitsRequest {
    // TODO: add others
    Orderbook(OrderBookRequest)
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OpenlimitsResponse {
    // TODO: add others
    Orderbook(OrderBookResponse)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SubscriptionMeta {
    Subscribe
}

/// Representation for core data for openlimits server process
pub struct OpenlimitsServer {
    pub exchange: Arc<Mutex<Nash>>, // generalize this to exchange in the future
    pub port: String
}

impl OpenlimitsServer {
    /// Start server loop, listening to and responding to connections
    pub fn start_socket_server(&self) -> Result<(), Box<dyn Error>>{
        let listener = TcpListener::bind(&format!("127.0.0.1:{}", self.port))?;
        // accept connections and process them serially
        for stream in listener.incoming() {
            let mut stream = stream?;
            let exchange = self.exchange.clone();
            std::thread::spawn(move || {
                let mut request = String::new();
                let _num_bytes_read = stream.read_to_string(&mut request).expect("failed to read");
                if let Ok(_) = serde_json::from_str::<SubscriptionMeta>(&request) {
                    // Here we feed back all subscription data over the connection
                    // First one byte with length (in bytes), then the payload
                }
                else if let Ok(ws_request) = serde_json::from_str::<OpenLimitsWebsocketMessage>(&request) {
                    let response = process_incoming_ws_request(ws_request, exchange);
                    // Here just respond OK, client will close it. subscruption will be retrieved over an independent connection
                    let response = format!("Started WS");
                    stream.write(response.as_bytes()).expect("failed to write response");
                }
                else if let Ok(openlimits_request) = serde_json::from_str::<OpenlimitsRequest>(&request) {
                    let response = process_incoming_request(openlimits_request, exchange);
                    // Here give back the response on this connection, then client will close it
                    let response = format!("Here is the response");
                    stream.write(response.as_bytes()).expect("failed to write response");
                }
                else {
                    stream.shutdown(Shutdown::Both).unwrap();
                }
                
            });
        }
        Ok(())

    }
}

fn process_incoming_ws_request(request: OpenLimitsWebsocketMessage, exchange: Arc<Mutex<Nash>>){
    // match on enum and thread through to appropriate Exchange trait method, then start subscription
    // subscription data will be retrieved on another connection 
}

fn process_incoming_request(request: OpenlimitsRequest, exchange: Arc<Mutex<Nash>>){
    // match on enum and thread through to appropriate Exchange trait method, then return result
}

#[cfg(test)]
mod tests {
    use super::SubscriptionMeta;
    #[test]
    fn serialize_request_for_subscription_stream() {
        let req = SubscriptionMeta::Subscribe;
        let sub_req = serde_json::to_string(&req).unwrap();
        println!("{:?}", sub_req);
    }
}
