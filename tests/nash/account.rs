use chrono::Duration;
use dotenv::dotenv;
use nash_native_client::ws_client::client::Environment;
use openlimits::{
    exchange::{ExchangeAccount, OpenLimits},
    model::OpenMarketOrderRequest,
    model::{
        CancelAllOrdersRequest, CancelOrderRequest, GetOrderHistoryRequest, OpenLimitOrderRequest,
        TimeInForce, TradeHistoryRequest,
    },
    nash::Nash,
    nash::NashCredentials,
    nash::NashParameters,
};
use rust_decimal::prelude::{Decimal, FromStr};
use std::env;
use std::time::Duration as NativeDuration;

#[tokio::test]
async fn limit_buy() {
    let exchange = init().await;
    let req = OpenLimitOrderRequest {
        time_in_force: TimeInForce::GoodTillCancelled,
        price: Decimal::from_str("100.46").expect("Couldn't parse string."),
        size: Decimal::from_str("0.10000").expect("Couldn't parse string."),
        market_pair: String::from("eth_usdc"),
        post_only: false,
    };
    let resp = exchange
        .limit_buy(&req)
        .await
        .expect("Couldn't parse string.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_buy_ioc() {
    let exchange = init().await;
    let req = OpenLimitOrderRequest {
        time_in_force: TimeInForce::ImmediateOrCancelled,
        price: Decimal::from_str("414.46").expect("Couldn't parse string."),
        size: Decimal::from_str("0.10000").expect("Couldn't parse string."),
        market_pair: String::from("eth_usdc"),
        post_only: false,
    };
    let resp = exchange
        .limit_buy(&req)
        .await
        .expect("Couldn't request limit buy.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_buy_fok() {
    let exchange = init().await;
    let req = OpenLimitOrderRequest {
        time_in_force: TimeInForce::FillOrKill,
        price: Decimal::from_str("414.46").expect("Couldn't parse string."),
        size: Decimal::from_str("0.10000").expect("Couldn't parse string."),
        market_pair: String::from("eth_usdc"),
        post_only: false,
    };
    let resp = exchange
        .limit_buy(&req)
        .await
        .expect("Couldn't request limit buy.");
    println!("{:?}", resp);
}

#[tokio::test]
#[ignore]
async fn limit_buy_ggt() {
    let exchange = init().await;
    let req = OpenLimitOrderRequest {
        time_in_force: TimeInForce::GoodTillTime(Duration::hours(2)),
        price: Decimal::from_str("414.46").expect("Couldn't parse string."),
        size: Decimal::from_str("0.02000").expect("Couldn't parse string."),
        market_pair: String::from("eth_usdc"),
        post_only: false,
    };
    let resp = exchange
        .limit_buy(&req)
        .await
        .expect("Couldn't request limit buy.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_buy() {
    let exchange = init().await;
    let req = OpenMarketOrderRequest {
        size: Decimal::from_str("10.0").expect("Couldn't parse string."),
        market_pair: String::from("usdc_eth"),
    };
    let resp = exchange
        .market_sell(&req)
        .await
        .expect("Couldn't request market buy.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_sell() {
    let exchange = init().await;
    let req = OpenMarketOrderRequest {
        size: Decimal::from_str("0.02").expect("Couldn't parse string."),
        market_pair: String::from("eth_usdc"),
    };
    let resp = exchange
        .market_sell(&req)
        .await
        .expect("Couldn't request market buy.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_sell() {
    let exchange = init().await;
    let req = OpenLimitOrderRequest {
        time_in_force: TimeInForce::GoodTillTime(Duration::hours(2)),
        price: Decimal::from_str("800.46").expect("Couldn't parse string."),
        size: Decimal::from_str("0.02").expect("Couldn't parse string."),
        market_pair: String::from("eth_usdc"),
        post_only: false,
    };
    let resp = exchange
        .limit_sell(&req)
        .await
        .expect("Couldn't request limit sell.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn cancel_order() {
    let exchange = init().await;
    let req = OpenLimitOrderRequest {
        time_in_force: TimeInForce::GoodTillCancelled,
        price: Decimal::from_str("200.46").expect("Couldn't parse string."),
        size: Decimal::from_str("0.10000").expect("Couldn't parse string."),
        market_pair: String::from("eth_usdc"),
        post_only: false,
    };
    let order = exchange
        .limit_buy(&req)
        .await
        .expect("Couldn't request limit buy.");

    let req = CancelOrderRequest {
        id: order.id,
        market_pair: Some(order.market_pair),
    };
    let resp = exchange
        .cancel_order(&req)
        .await
        .expect("Couldn't cancel order.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn cancel_all_orders() {
    let exchange = init().await;
    let req = OpenLimitOrderRequest {
        time_in_force: TimeInForce::GoodTillCancelled,
        price: Decimal::from_str("200.46").expect("Couldn't parse string."),
        size: Decimal::from_str("0.10000").expect("Couldn't parse string."),
        market_pair: String::from("eth_usdc"),
        post_only: false,
    };

    exchange
        .limit_sell(&req)
        .await
        .expect("Couldn't limit sell.");

    let req = CancelAllOrdersRequest {
        market_pair: Some("eth_btc".to_string()),
    };

    let resp = exchange
        .cancel_all_orders(&req)
        .await
        .expect("Couldn't cancel all orders.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_order_history() {
    let exchange = init().await;
    let req = GetOrderHistoryRequest {
        market_pair: Some(String::from("eth_btc")),
        paginator: None,
    };

    let resp = exchange
        .get_order_history(&req)
        .await
        .expect("Couldn't get order history.");
    println!("{:?}", resp);
}

// #[tokio::test]
// async fn get_all_open_orders() {
//     let mut exchange = init().await;
//     let req = OpenLimitOrderRequest {
//         time_in_force: TimeInForce::GoodTillCancelled,
//         price: Decimal::new(1, 1),
//         size: Decimal::new(2, 2),
//         market_pair: String::from("eth_btc"),
//     };
//     exchange.limit_sell(&req).await.expect("Couldn't limit sell.");

//     let resp = exchange.get_all_open_orders().await.expect("Couldn't get all open orders.");
//     println!("{:?}", resp);
// }

#[tokio::test]
async fn get_account_balances() {
    let exchange = init().await;
    let resp = exchange
        .get_account_balances(None)
        .await
        .expect("Couldn't get account balances.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_trade_history() {
    let exchange = init().await;
    let req = TradeHistoryRequest {
        market_pair: Some("eth_btc".to_string()),
        ..Default::default()
    };

    let resp = exchange
        .get_trade_history(&req)
        .await
        .expect("Couldn't get trade history.");
    println!("{:?}", resp);
}

async fn init() -> Nash {
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
        sign_states_loop_interval: None
    };

    OpenLimits::instantiate(parameters)
        .await
        .expect("Failed to create Client")
}
