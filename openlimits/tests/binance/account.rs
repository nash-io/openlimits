use dotenv::dotenv;
use std::env;

use openlimits::binance::Binance;
use openlimits::exchange::Exchange;
use openlimits::model::{OpenLimitOrderRequest, OpenMarketOrderRequest};
use rust_decimal::prelude::Decimal;

#[tokio::test]
async fn limit_buy() {
    let exchange = init();
    let req = OpenLimitOrderRequest {
        price: Decimal::new(1, 3),
        size: Decimal::new(1, 1),
        symbol: String::from("BNBBTC"),
    };
    let resp = exchange.limit_buy(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_sell() {
    let exchange = init();
    let req = OpenLimitOrderRequest {
        price: Decimal::new(1, 3),
        size: Decimal::new(1, 1),
        symbol: String::from("BNBBTC"),
    };
    let resp = exchange.limit_sell(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_buy() {
    let exchange = init();
    let req = OpenMarketOrderRequest {
        size: Decimal::new(1, 1),
        symbol: String::from("BNBBTC"),
    };
    let resp = exchange.market_buy(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_sell() {
    let exchange = init();
    let req = OpenMarketOrderRequest {
        size: Decimal::new(1, 1),
        symbol: String::from("BNBBTC"),
    };
    let resp = exchange.market_sell(&req).await.unwrap();
    println!("{:?}", resp);
}

fn init() -> Binance {
    dotenv().ok();
    Binance::with_credential(
        &env::var("BINANCE_API_KEY").unwrap(),
        &env::var("BINANCE_API_SECRET").unwrap(),
        true,
    )
}
