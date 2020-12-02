use futures::StreamExt;
use openlimits::coinbase::{client::websocket::CoinbaseWebsocket, model::websocket::{ChannelType, CoinbaseSubscription}, CoinbaseParameters};
use std::sync::mpsc::sync_channel;
use openlimits::coinbase::model::websocket::CoinbaseWebsocketMessage;
use openlimits::coinbase::model::websocket::Level2;
use openlimits::exchange_ws::ExchangeWs;
use openlimits::model::websocket::{WebSocketResponse, OpenLimitsWebSocketMessage, Subscription};
use openlimits::model::OrderBookResponse;

async fn test_subscription_callback(websocket: CoinbaseWebsocket, sub: CoinbaseSubscription, expected: OpenLimitsWebSocketMessage) {
    let (tx, rx) = sync_channel(0);

    websocket.subscribe(sub, move |message| {
        if let Ok(message) = message.as_ref() {
            if let WebSocketResponse::Generic(message) = message {
                assert_eq!(std::mem::discriminant(message), std::mem::discriminant(&expected));
                tx.send(()).expect("Couldn't send sync message.");
            }
        }
    }).await.expect("Couldn't subscribe.");
    rx.recv().expect("Couldn't receive sync message.");
}

#[tokio::test(core_threads = 2)]
async fn order_book() {
    let websocket = init().await;
    let sub = CoinbaseSubscription::Level2("BTC-USD".to_string());
    let expected = OpenLimitsWebSocketMessage::OrderBook(Default::default());
    test_subscription_callback(websocket, sub, expected).await;
}

async fn init() -> CoinbaseWebsocket {
    CoinbaseWebsocket::new(CoinbaseParameters {
        sandbox: true,
        credentials: None
    })
}