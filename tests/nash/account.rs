use crate::template::account;
use super::client::init_signed as init;

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
#[should_panic]
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
