use std::sync::mpsc::sync_channel;

use openlimits::{
    binance::client::websocket::BinanceWebsocket, exchange_ws::OpenLimitsWs,
    model::websocket::Subscription,
};

async fn test_subscription_callback(websocket: OpenLimitsWs<BinanceWebsocket>, sub: Subscription) {
    let (tx, rx) = sync_channel(0);

    websocket.subscribe(sub, move |m| {
        m.as_ref().expect("Couldn't get response.");
        tx.send(()).expect("Couldn't send sync message.");
    }).await.expect("Couldn't subscribe.");

    rx.recv().expect("Couldn't receive sync message.");
}

#[tokio::test(core_threads = 2)]
async fn orderbook() {
    let ws = init();
    let sub = Subscription::OrderBookUpdates("bnbbtc".to_string());
    test_subscription_callback(ws, sub).await;
}

#[tokio::test(core_threads = 2)]
async fn trades() {
    let ws = init();
    let sub = Subscription::Trades("btcusdt".to_string());
    test_subscription_callback(ws, sub).await;
}

fn init() -> OpenLimitsWs<BinanceWebsocket> {
    OpenLimitsWs {
        websocket: BinanceWebsocket::new(),
    }
}
