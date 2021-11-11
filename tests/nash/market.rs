use crate::template::market;
use super::client::init;

#[tokio::test]
async fn order_book() {
    market::order_book(&init().await).await;
}

#[tokio::test]
async fn get_price_ticker() {
    market::get_price_ticker(&init().await).await;
}

#[tokio::test]
async fn get_historic_rates() {
    market::get_historic_rates(&init().await).await;
}

#[tokio::test]
async fn pair() {
    market::pair(&init().await).await;
}
