use dotenv::dotenv;
use std::env;

use openlimits::coinbase::Coinbase;
use openlimits::exchange::Exchange;
use openlimits::model::{OpenLimitOrderRequest, OpenMarketOrderRequest};

#[tokio::test]
async fn limit_buy() {
    let exchange = init();
    let req = OpenLimitOrderRequest {
        price: 0.001,
        size: 0.1,
        symbol: String::from("ETH-BTC"),
    };
    let resp = exchange.limit_buy(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_sell() {
    let exchange = init();
    let req = OpenLimitOrderRequest {
        price: 0.1,
        size: 0.1,
        symbol: String::from("ETH-BTC"),
    };
    let resp = exchange.limit_sell(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_buy() {
    let exchange = init();
    let req = OpenMarketOrderRequest {
        size: 0.1,
        symbol: String::from("ETH-BTC"),
    };
    let resp = exchange.market_buy(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_sell() {
    let exchange = init();
    let req = OpenMarketOrderRequest {
        size: 0.1,
        symbol: String::from("ETH-BTC"),
    };
    let resp = exchange.market_sell(&req).await.unwrap();
    println!("{:?}", resp);
}

fn init() -> Coinbase {
    dotenv().ok();
    Coinbase::with_credential(
        &env::var("COINBASE_API_KEY").unwrap(),
        &env::var("COINBASE_API_SECRET").unwrap(),
        &env::var("COINBASE_PASSPHRASE").unwrap(),
        true,
    )
}
