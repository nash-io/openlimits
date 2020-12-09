use dotenv::dotenv;
use nash_native_client::ws_client::client::Environment;
use openlimits::{exchange_ws::OpenLimitsWs, model::websocket::Subscription, nash::NashWebsocket};
use std::{env, sync::mpsc::sync_channel};

async fn test_subscription_callback(websocket: OpenLimitsWs<NashWebsocket>, sub: Subscription) {
    websocket
        .subscribe(sub, move |message| {
            let message = message.as_ref().expect("Couldn't get response.");
            println!("{:#?}", message);
        })
        .await
        .expect("Couldn't subscribe.");
    loop {

    }
}

#[tokio::test(core_threads = 2)]
async fn orderbook() {
    let client = init().await;
    let sub = Subscription::OrderBookUpdates("btc_usdc".to_string());
    test_subscription_callback(client, sub).await;
}

#[tokio::test(core_threads = 2)]
async fn trades() {
    let client = init().await;
    let sub = Subscription::Trades("btc_usdc".to_string());
    test_subscription_callback(client, sub).await;
}

async fn init() -> OpenLimitsWs<NashWebsocket> {
    dotenv().ok();

    let websocket = NashWebsocket::with_credential(
        &env::var("NASH_API_SECRET").expect("Couldn't get environment variable."),
        &env::var("NASH_API_KEY").expect("Couldn't get environment variable."),
        1234,
        Environment::Production,
        10000,
    )
        .await;

    OpenLimitsWs { websocket  }
}
