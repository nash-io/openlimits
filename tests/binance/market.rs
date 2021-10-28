use openlimits::{
    OpenLimits,
    exchange::binance::Binance,
    exchange::binance::BinanceParameters,
};
use openlimits_exchange::exchange::Environment;
use crate::template::market;

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

async fn init() -> Binance {
    let parameters = BinanceParameters {
        credentials: None,
        environment: Environment::Sandbox,
    };

    OpenLimits::instantiate(parameters)
        .await
        .expect("Failed to create Client")
}
