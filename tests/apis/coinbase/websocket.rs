use openlimits::coinbase::{client::websocket::CoinbaseWebsocket, model::websocket::{CoinbaseSubscription}, CoinbaseParameters};
use std::sync::mpsc::sync_channel;
use openlimits::exchange_ws::ExchangeWs;
use openlimits::model::websocket::{WebSocketResponse, OpenLimitsWebSocketMessage};
use std::time::Duration;

async fn test_subscription_callback(websocket: CoinbaseWebsocket, sub: CoinbaseSubscription, expected_messages: Vec<OpenLimitsWebSocketMessage>) {
    let (tx, rx) = sync_channel(0);

    let mut received_messages: Vec<bool> = expected_messages.iter().map(|_| false).collect();

    websocket.subscribe(sub, move |message| {
        if let Ok(message) = message.as_ref() {
            if let WebSocketResponse::Generic(message) = message {
                let expected_iter = expected_messages.iter().map(|expected| {
                    std::mem::discriminant(expected) == std::mem::discriminant(&message)
                });
                for (already_received, currently_received) in received_messages.iter_mut().zip(expected_iter) {
                    if !*already_received {
                        *already_received = currently_received;
                    }
                }
                if received_messages.iter().all(|received| *received) {
                    tx.send(()).expect("Couldn't send sync message.");
                }
            }
        }
    }).await.expect("Couldn't subscribe.");
    rx.recv_timeout(Duration::from_secs(3)).expect("Couldn't receive sync message.");
}

#[tokio::test(core_threads = 2)]
async fn order_book() {
    let websocket = init().await;
    let sub = CoinbaseSubscription::Level2("BTC-USD".to_string());
    let expected = vec![
        OpenLimitsWebSocketMessage::OrderBook(Default::default()),
        OpenLimitsWebSocketMessage::OrderBookDiff(Default::default())
    ];
    test_subscription_callback(websocket, sub, expected).await;
}

async fn init() -> CoinbaseWebsocket {
    CoinbaseWebsocket::new(CoinbaseParameters {
        sandbox: true,
        credentials: None
    })
}