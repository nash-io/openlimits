use dotenv::dotenv;
use std::env;

use openlimits::{
    binance::Binance,
    binance::BinanceCredentials,
    binance::BinanceParameters,
    exchange::{ExchangeAccount, OpenLimits},
    model::{
        CancelAllOrdersRequest, CancelOrderRequest, GetOrderHistoryRequest, OpenLimitOrderRequest,
        OpenMarketOrderRequest, TimeInForce, TradeHistoryRequest,
    },
};
use rust_decimal::prelude::Decimal;

#[tokio::test]
async fn limit_buy() {
    let exchange = init().await;
    let req = OpenLimitOrderRequest {
        price: Decimal::new(17, 4),
        size: Decimal::new(1, 1),
        market_pair: String::from("BNBBTC"),
        post_only: false,
        time_in_force: TimeInForce::GoodTillCancelled,
    };
    let resp = exchange.limit_buy(&req).await.expect("Couldn't limit buy.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_sell() {
    let exchange = init().await;
    let req = OpenLimitOrderRequest {
        price: Decimal::new(1, 3),
        post_only: false,
        size: Decimal::new(1, 1),
        market_pair: String::from("BNBBTC"),
        time_in_force: TimeInForce::GoodTillCancelled,
    };
    let resp = exchange.limit_sell(&req).await.expect("Couldn't limit sell.");
    println!("{:?}", resp);
}

#[tokio::test]
#[ignore]
async fn post_only() {
    let exchange = init().await;
    /*let req = CancelAllOrdersRequest {
        market_pair: Some("BNBBTC".to_string()),
    };
    exchange.cancel_all_orders(&req).await.expect("Couldn't cancel all orders.");*/

    let req = OpenLimitOrderRequest {
        price: Decimal::new(1, 3),
        size: Decimal::new(1, 1),
        market_pair: String::from("BNBBTC"),
        post_only: true,
        time_in_force: TimeInForce::GoodTillCancelled,
    };
    let resp = exchange.limit_sell(&req).await.expect("Couldn't limit sell.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_buy() {
    let exchange = init().await;
    let req = OpenMarketOrderRequest {
        size: Decimal::new(1, 1),
        market_pair: String::from("BNBBTC"),
    };
    let resp = exchange.market_buy(&req).await.expect("Couldn't market buy.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_sell() {
    let exchange = init().await;
    let req = OpenMarketOrderRequest {
        size: Decimal::new(1, 1),
        market_pair: String::from("BNBBTC"),
    };
    let resp = exchange.market_sell(&req).await.expect("Couldn't market sell.");
    println!("{:?}", resp);
}

#[tokio::test]
#[ignore = "TODO fix"]
async fn cancel_order() {
    let exchange = init().await;
    let req = OpenLimitOrderRequest {
        price: Decimal::new(1, 3),
        size: Decimal::new(1, 1),
        market_pair: String::from("BNBBTC"),
        post_only: false,
        time_in_force: TimeInForce::GoodTillCancelled,
    };
    let order = exchange.limit_sell(&req).await.expect("Couldn't limit sell.");

    let req = CancelOrderRequest {
        id: order.id,
        market_pair: Some(order.market_pair),
    };

    let resp = exchange.cancel_order(&req).await.expect("Couldn't cancel order.");
    println!("{:?}", resp);
}

#[tokio::test]
#[ignore = "TODO fix"]
async fn cancel_all_orders() {
    let exchange = init().await;
    let req = OpenLimitOrderRequest {
        price: Decimal::new(1, 3),
        size: Decimal::new(1, 1),
        market_pair: String::from("BNBBTC"),
        post_only: false,
        time_in_force: TimeInForce::GoodTillCancelled,
    };
    exchange.limit_sell(&req).await.expect("Couldn't limit sell.");

    exchange.limit_sell(&req).await.expect("Couldn't limit sell.");

    let req = CancelAllOrdersRequest {
        market_pair: Some("BNBBTC".to_string()),
    };

    let resp = exchange.cancel_all_orders(&req).await.expect("Couldn't cancel all orders.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_order_history() {
    let exchange = init().await;
    let req = GetOrderHistoryRequest {
        market_pair: Some(String::from("BNBBTC")),
        paginator: None,
    };

    let resp = exchange.get_order_history(&req).await.expect("Couldn't get order history.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_all_open_orders() {
    let exchange = init().await;

    let req = OpenLimitOrderRequest {
        price: Decimal::new(5, 3),
        size: Decimal::new(1, 1),
        market_pair: String::from("BNBBTC"),
        post_only: false,
        time_in_force: TimeInForce::GoodTillCancelled,
    };
    exchange.limit_sell(&req).await.expect("Couldn't limit sell.");

    let resp = exchange.get_all_open_orders().await.expect("Couldn't get all open orders.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_account_balances() {
    let exchange = init().await;

    let resp = exchange.get_account_balances(None).await.expect("Couldn't get acount balances.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_trade_history() {
    let exchange = init().await;
    let req = TradeHistoryRequest {
        market_pair: Some("BNBBTC".to_string()),
        ..Default::default()
    };

    let resp = exchange.get_trade_history(&req).await.expect("Couldn't get trade history.");
    println!("{:?}", resp);
}

async fn init() -> Binance {
    dotenv().ok();

    let parameters = BinanceParameters {
        credentials: Some(BinanceCredentials {
            api_key: env::var("BINANCE_API_KEY").expect("Couldn't get environment variable."),
            api_secret: env::var("BINANCE_API_SECRET").expect("Couldn't get environment variable."),
        }),
        sandbox: true,
    };

    OpenLimits::instantiate(parameters).await
}
