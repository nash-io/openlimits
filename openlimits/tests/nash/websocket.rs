use dotenv::dotenv;
use futures::StreamExt;
use nash_native_client::ws_client::client::Client;
use openlimits::{exchange_ws::OpenLimitsWs, model::websocket::Subscription, nash::NashStream};
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

    let ws = Client::from_key_data(
        &env::var("NASH_API_SECRET").unwrap(),
        &env::var("NASH_API_KEY").unwrap(),
        None,
        1234,
        nash_native_client::ws_client::client::Environment::Production,
        100000,
    )
    .await
    .unwrap();

    OpenLimitsWs {
        websocket: NashStream { client: ws },
    }
}
