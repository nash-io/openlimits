use crate::template::callbacks;
use super::client::init_ws as init;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn orderbook() {
    callbacks::orderbook(&init().await).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn trades() {
    callbacks::trades(&init().await).await;
}