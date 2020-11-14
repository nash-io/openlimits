use std::sync::mpsc::sync_channel;

use openlimits::{
    binance::client::websocket::BinanceWebsocket, exchange_ws::OpenLimitsWs,
    model::websocket::Subscription,
};

#[tokio::test(core_threads = 2)]
async fn orderbook() {
    let (tx, rx) = sync_channel(0);
    let ws = init();
    let sub = Subscription::OrderBook("bnbbtc".to_string());
    ws.subscribe(sub, move |m| {
        println!("{:?}", m);
        tx.send(()).unwrap();
    })
    .await
    .unwrap();

    rx.recv().unwrap();
}

#[tokio::test(core_threads = 2)]
async fn trades() {
    let (tx, rx) = sync_channel(0);
    let ws = init();
    let sub = Subscription::Trades("btcusdt".to_string());
    ws.subscribe(sub, move |m| {
        println!("{:?}", m);
        tx.send(()).unwrap();
    })
    .await
    .unwrap();

    rx.recv().unwrap();
}

fn init() -> OpenLimitsWs<BinanceWebsocket> {
    OpenLimitsWs {
        websocket: BinanceWebsocket::new(),
    }
}
