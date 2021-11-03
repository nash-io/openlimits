use crate::template::streams;
use super::client::init_ws as init;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn orderbook() {
    streams::orderbook(&init().await).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn trades() {
    streams::trades(&init().await).await;
}
