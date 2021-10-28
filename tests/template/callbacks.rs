use std::sync::mpsc::sync_channel;

use openlimits::model::websocket::Subscription;
use openlimits::exchange::traits::stream::ExchangeWs;

async fn test_subscription_callback(websocket: &impl ExchangeWs, sub: Subscription) {
    let (tx, rx) = sync_channel(0);

    websocket
        .subscribe(sub, move |m| {
            m.as_ref().expect("Couldn't get response.");
            tx.send(()).expect("Couldn't send sync message.");
        })
        .await
        .expect("Couldn't subscribe.");

    rx.recv().expect("Couldn't receive sync message.");
}

pub async fn orderbook(ws: &impl ExchangeWs) {
    let sub = Subscription::OrderBookUpdates("bnbbtc".to_string());
    test_subscription_callback(ws, sub).await;
}

pub async fn trades(ws: &impl ExchangeWs) {
    let sub = Subscription::Trades("btcusdt".to_string());
    test_subscription_callback(ws, sub).await;
}
