use dotenv::dotenv;
use openlimits::{exchange_ws::OpenLimitsWs, model::websocket::Subscription, nash::NashStream};
use std::env;

#[tokio::test]
async fn orderbook() {
    let client = init().await;
    let sub = Subscription::OrderBookUpdates("btc_usdc".to_string());
    client
        .subscribe(sub, |m| println!("{:?}", m))
        .await
        .unwrap();
}

#[tokio::test]
async fn trades() {
    let client = init().await;
    let sub = Subscription::Trades("btc_usdc".to_string());
    client
        .subscribe(sub, |m| println!("{:?}", m))
        .await
        .unwrap();
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
