use openlimits::exchange::binance::{BinanceParameters, client::websocket::BinanceWebsocket};
use openlimits::exchange::traits::stream::ExchangeWs;
use crate::template::callbacks;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn orderbook() {
    callbacks::orderbook(&init().await).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn trades() {
    callbacks::trades(&init().await).await;
}

async fn init() -> BinanceWebsocket {
    BinanceWebsocket::new(BinanceParameters::production())
        .await
        .expect("Failed to create Client")
}
