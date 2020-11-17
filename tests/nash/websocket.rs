use dotenv::dotenv;
use openlimits::{exchange_ws::OpenLimitsWs, model::websocket::Subscription, nash::NashStream};
use std::{env, sync::mpsc::sync_channel};

#[tokio::test(core_threads = 2)]
async fn orderbook() {
    let (tx, rx) = sync_channel(0);
    let client = init().await;
    let sub = Subscription::OrderBookUpdates("btc_usdc".to_string());
    client
        .subscribe(sub, move |m| {
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
    let client = init().await;
    let sub = Subscription::Trades("btc_usdc".to_string());
    client
        .subscribe(sub, move |m| {
            println!("{:?}", m);
            tx.send(()).unwrap();
        })
        .await
        .unwrap();

    rx.recv().unwrap();
}

async fn init() -> OpenLimitsWs<NashStream> {
    dotenv().ok();

    let websocket = NashStream::with_credential(
        &env::var("NASH_API_SECRET").unwrap(),
        &env::var("NASH_API_KEY").unwrap(),
        1234,
        true,
        10000,
    )
    .await;

    OpenLimitsWs { websocket }
}
