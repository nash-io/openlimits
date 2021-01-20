use crate::openlimits::exchange::ExchangeAccount;
use dotenv::dotenv;
use nash_native_client::ws_client::client::Environment;
use openlimits::exchange::OpenLimits;
use openlimits::model::{CancelAllOrdersRequest, OpenLimitOrderRequest, TimeInForce};
use openlimits::nash::{Nash, NashCredentials, NashParameters};
use openlimits::{exchange_ws::OpenLimitsWs, model::websocket::Subscription, nash::NashWebsocket};
use rust_decimal::Decimal;
use std::str::FromStr;
use std::time::Duration as NativeDuration;
use std::{env, sync::mpsc::sync_channel};
use tokio::time::Duration;

async fn init_exchange() -> Nash {
    dotenv().ok();

    let parameters = NashParameters {
        affiliate_code: None,
        credentials: Some(NashCredentials {
            secret: env::var("NASH_API_SECRET").expect("Couldn't get environment variable."),
            session: env::var("NASH_API_KEY").expect("Couldn't get environment variable."),
        }),
        environment: Environment::Sandbox,
        client_id: 1,
        timeout: NativeDuration::new(10, 0),
        sign_states_loop_interval: None,
    };

    OpenLimits::instantiate(parameters)
        .await
        .expect("Failed to create Client")
}

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

async fn test_account_subscription_callback(
    websocket: OpenLimitsWs<NashWebsocket>,
    sub: Subscription,
    cancel_orders: bool,
) {
    let (tx, rx) = sync_channel(0);

    websocket
        .subscribe(sub, move |m| {
            m.as_ref().expect("Couldn't get response.");
            tx.send(()).expect("Couldn't send sync message.");
        })
        .await
        .expect("Couldn't subscribe.");

    let exchange = init_exchange().await;
    let req = OpenLimitOrderRequest {
        time_in_force: TimeInForce::GoodTillCancelled,
        price: Decimal::from_str("0.01").expect("Couldn't parse string."),
        size: Decimal::from_str("0.1").expect("Couldn't parse string."),
        market_pair: String::from("eth_btc"),
        post_only: false,
    };

    exchange
        .limit_buy(&req)
        .await
        .expect("Couldn't limit sell.");

    if cancel_orders {
        let req = CancelAllOrdersRequest {
            market_pair: Some("eth_btc".to_string()),
        };

        let resp = exchange
            .cancel_all_orders(&req)
            .await
            .expect("Couldn't cancel all orders.");
        println!("{:?}", resp);
    }

    rx.recv().expect("Couldn't receive sync message.");
}

#[tokio::test(core_threads = 2)]
async fn account_orders() {
    let client = init().await;
    let sub = Subscription::AccountOrders("eth_btc".to_string());
    test_account_subscription_callback(client, sub, true).await;
}

#[tokio::test(core_threads = 2)]
async fn account_trades() {
    let client = init().await;
    let sub = Subscription::AccountTrades("eth_btc".to_string());
    test_account_subscription_callback(client, sub, false).await;
}

#[tokio::test(core_threads = 2)]
async fn account_balance() {
    let client = init().await;
    let sub = Subscription::AccountBalance("eth".to_string());
    test_account_subscription_callback(client, sub, false).await;
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
        Environment::Sandbox,
        Duration::new(10, 0),
        None,
    )
    .await
    .expect("Couldn't connect.");

    OpenLimitsWs { websocket }
}
