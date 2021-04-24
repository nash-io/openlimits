use futures::stream::StreamExt;
use openlimits::{
    exchange::binance::{BinanceParameters, BinanceWebsocket},
    exchange_ws::{ExchangeWs, OpenLimitsWs},
    model::websocket::Subscription,
};

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn orderbook() {
    let ws = init().await;
    let s = ws
        .create_stream(&[Subscription::OrderBookUpdates("bnbbtc".to_string())])
        .await;

    let ob = s.expect("Couldn't create stream.").next().await;

    print!("{:?}", ob);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn trades() {
    let ws = init().await;
    let s = ws
        .create_stream(&[Subscription::Trades("bnbbtc".to_string())])
        .await;

    let trades = s.expect("Couldn't create stream.").next().await;

    print!("{:?}", trades);
}

async fn init() -> OpenLimitsWs<BinanceWebsocket> {
    OpenLimitsWs {
        websocket: BinanceWebsocket::new(BinanceParameters::prod())
            .await
            .expect("Failed to create Client"),
    }
}
