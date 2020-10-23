use std::marker::PhantomData;

use futures::StreamExt;
use openlimits::{
    binance::{client::websocket::BinanceWebsocket, Binance},
    exchange::Exchange,
    exchange_ws::OpenLimitsWs,
    model::websocket::Subscription,
};

#[tokio::test]
async fn orderbook() {
    let mut ws = init();
    let sub = Subscription::OrderBook("bnbbtc".to_string(), 5);
    ws.subscribe(sub).await.unwrap();
    let v = ws.next().await;
    println!("{:?}", v);
}

#[tokio::test]
async fn trades() {
    let mut ws = init();
    let sub = Subscription::Trade("btcusdt".to_string());
    ws.subscribe(sub).await.unwrap();
    let v = ws.next().await;
    println!("{:?}", v);
}

fn init() -> OpenLimitsWs<BinanceWebsocket, Exchange<Binance>> {
    OpenLimitsWs {
        websocket: BinanceWebsocket::new(),
        phantom: PhantomData,
    }
}
