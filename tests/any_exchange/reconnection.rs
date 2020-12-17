use dotenv::dotenv;
use nash_native_client::ws_client::client::Environment;
use openlimits::shared::Result;
use openlimits::{model::websocket::Subscription, nash::NashWebsocket};
use std::env;
use tokio::time::Duration;

use openlimits::nash::{NashCredentials, NashParameters};
use openlimits::reconnectable_ws::ReconnectableWebsocket;
use std::thread::sleep;

// FIXME: We need to create a disconnection mechanism to properly test this feature.
async fn test_subscription_callback(
    websocket: ReconnectableWebsocket<NashWebsocket>,
    sub: Subscription,
) {
    websocket
        .subscribe(sub, |message| match message.as_ref() {
            Ok(message) => println!("{:#?}", message),
            Err(e) => println!("Disconnected: {:#?}", e),
        })
        .await
        .expect("Couldn't subscribe");
    sleep(Duration::from_secs_f32(3.0));
}

#[tokio::test(core_threads = 2)]
async fn orderbook() {
    let client = init().await;
    let sub = Subscription::OrderBookUpdates("btc_usdc".to_string());
    test_subscription_callback(client.expect("Couldn't create client."), sub).await;
}

#[tokio::test(core_threads = 2)]
async fn trades() {
    let client = init().await.expect("Couldn't create client.");
    let sub = Subscription::Trades("btc_usdc".to_string());
    test_subscription_callback(client, sub).await;
}

async fn init() -> Result<ReconnectableWebsocket<NashWebsocket>> {
    dotenv().ok();
    ReconnectableWebsocket::instantiate(
        NashParameters {
            timeout: Duration::from_secs_f32(2.0),
            client_id: 123,
            credentials: Some(NashCredentials {
                secret: env::var("NASH_API_SECRET").expect("Couldn't get environment variable."),
                session: env::var("NASH_API_KEY").expect("Couldn't get environment variable."),
            }),
            affiliate_code: None,
            environment: Environment::Production,
        },
        Duration::from_secs_f32(1.0),
    )
    .await
}
