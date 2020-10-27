use dotenv::dotenv;
use futures::StreamExt;
use openlimits::{
    exchange::Exchange, exchange_ws::OpenLimitsWs, model::websocket::Subscription, nash::Nash,
    nash::NashStream,
};
use std::env;

#[tokio::test]
async fn orderbook() {
    let mut client = init().await;
    let sub = Subscription::OrderBook("btc_usdc".to_string(), 5);
    client.subscribe(sub).await.unwrap();

    let item = client.next().await;
    println!("{:?}", item.unwrap().unwrap());
}

#[tokio::test]
async fn trades() {
    let mut client = init().await;
    let sub = Subscription::Trade("btc_usdc".to_string());
    client.subscribe(sub).await.unwrap();

    let item = client.next().await;
    println!("{:?}", item.unwrap().unwrap());
}

async fn init() -> OpenLimitsWs<NashStream> {
    dotenv().ok();

    let websocket = NashStream::with_credential(
        &env::var("NASH_API_SECRET").unwrap(),
        &env::var("NASH_API_KEY").unwrap(),
        1234,
        false,
        10000,
    )
    .await;

    OpenLimitsWs { websocket }
}
