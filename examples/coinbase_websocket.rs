use openlimits::exchange::traits::stream::OpenLimitsWs;
use openlimits::exchange::coinbase::client::websocket::CoinbaseWebsocket;
use openlimits::exchange::coinbase::CoinbaseParameters;
use openlimits::model::websocket::OpenLimitsWebSocketMessage::OrderBook;
use openlimits::model::websocket::Subscription::OrderBookUpdates;
use openlimits::model::websocket::WebSocketResponse::Generic;


#[tokio::main]
async fn main() {
    let coinbase_websocket = OpenLimitsWs {
        websocket: CoinbaseWebsocket::new(CoinbaseParameters::prod())
    };

    coinbase_websocket.subscribe(OrderBookUpdates("BTC-EUR".to_string()), move |m| {
        let r = m.as_ref();

        if let Ok(Generic(OrderBook(order_book))) = r {
            println!("{:?}", order_book)
        } else if let Err(err) = r {
            println!("{:#?}", err);
        }
    })
    .await
    .expect("Failed to subscribe to orderbook on Coinbase");

    std::thread::sleep(std::time::Duration::from_millis(5000));
}