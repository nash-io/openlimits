use dotenv::dotenv;
use nash_native_client::Environment;
use openlimits::exchange_ws::ExchangeWs;
use openlimits::nash::{NashCredentials, NashParameters};
use openlimits::{exchange_ws::OpenLimitsWs, model::websocket::Subscription, nash::NashWebsocket};
use std::{env, sync::mpsc::sync_channel};
use tokio::time::Duration;

async fn test_subscription_callback(websocket: OpenLimitsWs<NashWebsocket>, sub: Subscription) {
    let (tx, rx) = sync_channel(0);

    websocket
        .subscribe(sub, move |m| {
            m.as_ref().expect("Couldn't get response.");
            tx.send(()).expect("Couldn't send sync message.");
        })
        .await
        .expect("Couldn't subscribe.");

    rx.recv().expect("Couldn't receive sync message.");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn orderbook() {
    let client = init().await;
    let sub = Subscription::OrderBookUpdates("btc_usdc".to_string());
    test_subscription_callback(client, sub).await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn trades() {
    let client = init().await;
    let sub = Subscription::Trades("btc_usdc".to_string());
    test_subscription_callback(client, sub).await;
}

async fn init() -> OpenLimitsWs<NashWebsocket> {
    dotenv().ok();

    let websocket = NashWebsocket::new(NashParameters {
        credentials: Some(NashCredentials {
            secret: env::var("NASH_API_SECRET").expect("Couldn't get environment variable."),
            session: env::var("NASH_API_KEY").expect("Couldn't get environment variable."),
        }),
        affiliate_code: None,
        client_id: 1234,
        environment: Environment::Sandbox,
        timeout: Duration::from_secs(10),
        sign_states_loop_interval: None,
    })
    .await
    .expect("Couldn't connect.");

    OpenLimitsWs { websocket }
}
