use dotenv::dotenv;
use std::env;

use crate::template::account;

use openlimits::{
    OpenLimits,
    exchange::binance::Binance,
    exchange::binance::BinanceCredentials,
    exchange::binance::BinanceParameters,
};
use openlimits_exchange::exchange::Environment;

#[tokio::test]
async fn limit_buy() {
    account::limit_buy(&init().await).await;
}

#[tokio::test]
async fn limit_sell() {
    account::limit_sell(&init().await).await;
}

#[tokio::test]
async fn post_only() {
    account::post_only(&init().await).await;
}

#[tokio::test]
async fn market_buy() {
    account::market_buy(&init().await).await;
}

#[tokio::test]
async fn market_sell() {
    account::market_sell(&init().await).await;
}

#[tokio::test]
async fn cancel_order() {
    account::cancel_order(&init().await).await;
}

#[tokio::test]
async fn cancel_all_orders() {
    account::cancel_all_orders(&init().await).await;
}

#[tokio::test]
async fn get_order_history() {
    account::get_order_history(&init().await).await;
}

#[tokio::test]
async fn get_all_open_orders() {
    account::get_all_open_orders(&init().await).await;
}

#[tokio::test]
async fn get_account_balances() {
    account::get_account_balances(&init().await).await;
}

#[tokio::test]
async fn get_trade_history() {
    account::get_trade_history(&init().await).await;
}

async fn init() -> Binance {
    dotenv().ok();

    let parameters = BinanceParameters {
        credentials: Some(BinanceCredentials {
            api_key: env::var("BINANCE_API_KEY").expect("Couldn't get environment variable."),
            api_secret: env::var("BINANCE_API_SECRET").expect("Couldn't get environment variable."),
        }),
        environment: Environment::Sandbox,
    };

    OpenLimits::instantiate(parameters)
        .await
        .expect("Failed to create Client")
}
