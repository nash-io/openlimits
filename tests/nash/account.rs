use dotenv::dotenv;
use nash_native_client::ws_client::client::Environment;
use std::env;

use openlimits::{
    exchange::ExchangeWrapper,
    exchange::OpenLimits,
    model::{
        CancelAllOrdersRequest, CancelOrderRequest, GetOrderHistoryRequest, OpenLimitOrderRequest,
        TradeHistoryRequest,
    },
    nash::Nash,
    nash::NashCredentials,
    nash::NashParameters,
};
use rust_decimal::prelude::Decimal;

#[tokio::test]
async fn limit_buy() {
    let exchange = init().await;
    let req = OpenLimitOrderRequest {
        price: Decimal::new(1, 3),
        size: Decimal::new(1, 1),
        market_pair: String::from("eth_btc"),
    };
    let resp = exchange.limit_buy(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_sell() {
    let exchange = init().await;
    let req = OpenLimitOrderRequest {
        price: Decimal::new(1, 1),
        size: Decimal::new(2, 2),
        market_pair: String::from("eth_btc"),
    };
    let resp = exchange.limit_sell(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn cancel_order() {
    let exchange = init().await;
    let req = OpenLimitOrderRequest {
        price: Decimal::new(1, 1),
        size: Decimal::new(2, 2),
        market_pair: String::from("eth_btc"),
    };
    let order = exchange.limit_sell(&req).await.unwrap();

    let req = CancelOrderRequest {
        id: order.id,
        market_pair: Some(order.market_pair),
    };
    let resp = exchange.cancel_order(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn cancel_all_orders() {
    let exchange = init().await;
    let req = OpenLimitOrderRequest {
        price: Decimal::new(1, 1),
        size: Decimal::new(2, 2),
        market_pair: String::from("eth_btc"),
    };
    exchange.limit_sell(&req).await.unwrap();

    exchange.limit_sell(&req).await.unwrap();

    let req = CancelAllOrdersRequest {
        market_pair: Some("eth_btc".to_string()),
    };

    let resp = exchange.cancel_all_orders(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_order_history() {
    let exchange = init().await;
    let req = GetOrderHistoryRequest {
        market_pair: Some(String::from("eth_btc")),
        paginator: None,
    };

    let resp = exchange.get_order_history(&req).await.unwrap();
    println!("{:?}", resp);
}

// #[tokio::test]
// async fn get_all_open_orders() {
//     let mut exchange = init().await;
//     let req = OpenLimitOrderRequest {
//         price: Decimal::new(1, 1),
//         size: Decimal::new(2, 2),
//         market_pair: String::from("eth_btc"),
//     };
//     exchange.limit_sell(&req).await.unwrap();

//     let resp = exchange.get_all_open_orders().await.unwrap();
//     println!("{:?}", resp);
// }

#[tokio::test]
async fn get_account_balances() {
    let exchange = init().await;
    let resp = exchange.get_account_balances(None).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_trade_history() {
    let exchange = init().await;
    let req = TradeHistoryRequest {
        market_pair: Some("eth_btc".to_string()),
        ..Default::default()
    };

    let resp = exchange.get_trade_history(&req).await.unwrap();
    println!("{:?}", resp);
}

async fn init() -> ExchangeWrapper<Nash> {
    dotenv().ok();

    let parameters = NashParameters {
        credentials: Some(NashCredentials {
            secret: env::var("NASH_API_SECRET").unwrap(),
            session: env::var("NASH_API_KEY").unwrap(),
        }),
        environment: Environment::Production,
        client_id: 1234,
        timeout: 100000,
    };

    OpenLimits::instantiate(parameters).await
}
