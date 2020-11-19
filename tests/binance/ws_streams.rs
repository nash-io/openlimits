use futures::stream::StreamExt;
use openlimits::{
    binance::{BinanceParameters, BinanceWebsocket},
    exchange_ws::{ExchangeWs, OpenLimitsWs},
    model::websocket::Subscription,
};

#[tokio::test(core_threads = 2)]
async fn orderbook() {
    let ws = init().await;
    let s = ws
        .create_stream(&[Subscription::OrderBookUpdates("bnbbtc".to_string())])
        .await;

    assert!(s.is_ok());

    let ob = s.unwrap().next().await;

    print!("{:?}", ob);
}

#[tokio::test(core_threads = 2)]
async fn trades() {
    let ws = init().await;
    let s = ws
        .create_stream(&[Subscription::Trades("bnbbtc".to_string())])
        .await;

    assert!(s.is_ok());

    let trades = s.unwrap().next().await;

    print!("{:?}", trades);
}

async fn init() -> OpenLimitsWs<BinanceWebsocket> {
    OpenLimitsWs {
        websocket: BinanceWebsocket::new(BinanceParameters::prod()).await,
    }
}
