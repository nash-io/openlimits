use openlimits::{
    binance::client::websocket::BinanceWebsocket, exchange_ws::OpenLimitsWs,
    model::websocket::Subscription,
};

#[tokio::test]
async fn orderbook() {
    let ws = init();
    let sub = Subscription::OrderBook("bnbbtc".to_string());
    ws.subscribe(sub, |m| println!("{:?}", m)).await.unwrap();
}

#[tokio::test]
async fn trades() {
    let ws = init();
    let sub = Subscription::Trades("btcusdt".to_string());
    ws.subscribe(sub, |m| println!("{:?}", m)).await.unwrap();
}

fn init() -> OpenLimitsWs<BinanceWebsocket> {
    OpenLimitsWs {
        websocket: BinanceWebsocket::new(),
    }
}
