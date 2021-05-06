use openlimits::exchange::traits::stream::OpenLimitsWs;
use openlimits::exchange::binance::BinanceWebsocket;
use openlimits::exchange::binance::BinanceParameters;
use openlimits::prelude::*;
use openlimits::model::websocket::OpenLimitsWebSocketMessage::OrderBook;
use openlimits::model::websocket::Subscription::OrderBookUpdates;
use openlimits::model::websocket::WebSocketResponse::Generic;

#[tokio::main]
async fn main() {
    let binance_websocket = OpenLimitsWs {
        websocket: BinanceWebsocket::new(BinanceParameters::prod())
            .await
            .expect("Failed to create Client")
    };

    binance_websocket.subscribe(OrderBookUpdates("btceur".to_string()), move |m| {
        let r = m.as_ref();

        if let Ok(Generic(OrderBook(order_book))) = r {
            println!("{:?}", order_book)
        } else if let Err(err) = r {
            println!("{:#?}", err);
        }
    })
    .await
    .expect("Failed to subscribe to orderbook on Binance");

    std::thread::sleep(std::time::Duration::from_millis(5000));
}