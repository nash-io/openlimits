use dotenv::dotenv;
use std::env;

use openlimits::openlimits::{
    coinbase::Coinbase,
    exchange::Exchange,
    model::{
        CancelAllOrdersRequest, CancelOrderRequest, GetOrderHistoryRequest, OpenLimitOrderRequest,
        OpenMarketOrderRequest, TradeHistoryRequest,
    },
};
use rust_decimal::prelude::Decimal;

#[tokio::test]
async fn limit_buy() {
    let exchange = init();
    let req = OpenLimitOrderRequest {
        price: Decimal::new(1, 3),
        size: Decimal::new(1, 1),
        symbol: String::from("ETH-BTC"),
    };
    let resp = exchange.limit_buy(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_sell() {
    let exchange = init();
    let req = OpenLimitOrderRequest {
        price: Decimal::new(1, 1),
        size: Decimal::new(1, 1),
        symbol: String::from("ETH-BTC"),
    };
    let resp = exchange.limit_sell(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_buy() {
    let exchange = init();
    let req = OpenMarketOrderRequest {
        size: Decimal::new(1, 1),
        symbol: String::from("ETH-BTC"),
    };
    let resp = exchange.market_buy(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_sell() {
    let exchange = init();
    let req = OpenMarketOrderRequest {
        size: Decimal::new(1, 1),
        symbol: String::from("ETH-BTC"),
    };
    let resp = exchange.market_sell(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn cancel_order() {
    let exchange = init();
    let req = OpenLimitOrderRequest {
        price: Decimal::new(1, 1),
        size: Decimal::new(1, 1),
        symbol: String::from("ETH-BTC"),
    };
    let order = exchange.limit_sell(&req).await.unwrap();

    let req = CancelOrderRequest {
        id: order.id,
        pair: Some(order.symbol),
    };
    let resp = exchange.cancel_order(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn cancel_all_orders() {
    let exchange = init();
    let req = OpenLimitOrderRequest {
        price: Decimal::new(1, 1),
        size: Decimal::new(1, 1),
        symbol: String::from("ETH-BTC"),
    };
    exchange.limit_sell(&req).await.unwrap();

    exchange.limit_sell(&req).await.unwrap();

    let req = CancelAllOrdersRequest {
        pair: Some("ETH-BTC".to_string()),
    };

    let resp = exchange.cancel_all_orders(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_order_history() {
    let exchange = init();
    let req = GetOrderHistoryRequest {
        symbol: Some(String::from("ETH-BTC")),
        paginator: None,
    };

    let resp = exchange.get_order_history(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_all_open_orders() {
    let exchange = init();
    let req = OpenLimitOrderRequest {
        price: Decimal::new(1, 1),
        size: Decimal::new(1, 1),
        symbol: String::from("ETH-BTC"),
    };
    exchange.limit_sell(&req).await.unwrap();

    let resp = exchange.get_all_open_orders().await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_account_balances() {
    let exchange = init();
    let resp = exchange.get_account_balances(None).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_trade_history() {
    let exchange = init();
    let req = TradeHistoryRequest {
        pair: Some("ETH-BTC".to_string()),
        ..Default::default()
    };

    let resp = exchange.get_trade_history(&req).await.unwrap();
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
