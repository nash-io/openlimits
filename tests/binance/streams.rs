use openlimits::{
    exchange::binance::{BinanceParameters, BinanceWebsocket},
};
use openlimits::exchange::traits::stream::ExchangeWs;

use crate::template::streams;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn orderbook() {
    streams::orderbook(&init().await).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn trades() {
    streams::trades(&init().await).await;
}

async fn init() -> BinanceWebsocket {
    BinanceWebsocket::new(BinanceParameters::production())
        .await
        .expect("Failed to create Binance stream.")
}
