use nash_native_client::ws_client::client::Environment;
use openlimits::shared::Result;
use openlimits::{model::websocket::Subscription, nash::NashWebsocket};
use tokio::time::Duration;

use openlimits::nash::NashParameters;
use openlimits::coinbase::CoinbaseParameters;
use openlimits::binance::{BinanceParameters, BinanceWebsocket};
use openlimits::reconnectable_ws::ReconnectableWebsocket;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::sync_channel;
use openlimits::coinbase::client::websocket::CoinbaseWebsocket;
use openlimits::exchange_ws::ExchangeWs;

async fn test_subscription_callback<E: ExchangeWs + 'static>(websocket: ReconnectableWebsocket<E>, sub: Subscription) {
    let disconnections = Arc::new(Mutex::new(0 as u32));
    let (tx, rx) = sync_channel(0);
    let websocket = Arc::new(websocket);
    let weak_websocket = Arc::downgrade(&websocket);
    websocket.subscribe(sub, move |message| {
        match message.as_ref() {
            Ok(_message) => {
                if let Ok(disconnections) = disconnections.lock().map(|value| *value) {
                    if disconnections >= 2 {
                        tx.send(()).expect("Couldn't send sync.");
                    }
                }
                let websocket = weak_websocket.upgrade().expect("Couldn't get websocket.");
                tokio::spawn(async move {
                    websocket.disconnect().await
                });
            },
            Err(_error) => {
                *disconnections.lock().expect("Couldn't lock disconnections.") += 1;
            }
        }
    }).await.expect("Couldn't subscribe");
    rx.recv_timeout(Duration::from_secs_f32(10.0)).expect("Couldn't receive sync.");
}

#[tokio::test(core_threads = 2)]
async fn coinbase() {
    let client = init_coinbase().await;
    let sub = Subscription::OrderBookUpdates("BTC-USD".to_string());
    test_subscription_callback(client.expect("Couldn't create client."), sub).await;
}

#[tokio::test(core_threads = 2)]
async fn nash() {
    let client = init_nash().await;
    let sub = Subscription::OrderBookUpdates("btc_usdc".to_string());
    test_subscription_callback(client.expect("Couldn't create client."), sub).await;
}

#[tokio::test(core_threads = 2)]
async fn binance() {
    let client = init_binance().await;
    let sub = Subscription::OrderBookUpdates("bnbbtc".to_string());
    test_subscription_callback(client.expect("Couldn't create client."), sub).await;
}

async fn init_coinbase() -> Result<ReconnectableWebsocket<CoinbaseWebsocket>> {
    ReconnectableWebsocket::instantiate(
        CoinbaseParameters {
            credentials: None,
            sandbox: true
        },
        Duration::from_secs_f32(1.0)
    ).await
}

async fn init_nash() -> Result<ReconnectableWebsocket<NashWebsocket>> {
    ReconnectableWebsocket::instantiate(
        NashParameters {
            timeout: Duration::from_secs_f32(2.0),
            client_id: 123,
            credentials: None,
            affiliate_code: None,
            environment: Environment::Production,
        },
        Duration::from_secs_f32(1.0),
    )
        .await
}

async fn init_binance() -> Result<ReconnectableWebsocket<BinanceWebsocket>> {
    ReconnectableWebsocket::instantiate(
        BinanceParameters {
            sandbox: false,
            credentials: None
        },
        Duration::from_secs_f32(1.0)
    ).await
}