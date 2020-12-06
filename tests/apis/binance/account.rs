use dotenv::dotenv;
use rust_decimal::prelude::Decimal;
use std::env;

use openlimits::{
    binance::{
        model::{AllOrderReq, TimeInForce, TradeHistoryReq},
        Binance, BinanceCredentials, BinanceParameters,
    },
    exchange::Exchange,
    exchange_info::ExchangeInfoRetrieval,
};

#[tokio::test]
async fn get_account() {
    let exchange = init().await;
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner time.")
        .get_account()
        .await
        .expect("Couldn't get account");
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_balance() {
    let exchange = init().await;
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner time.")
        .get_balance("BTC")
        .await
        .expect("Couldn't get balance.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_open_orders() {
    let exchange = init().await;
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner time.")
        .get_open_orders("BNBBTC")
        .await
        .expect("Couldn't get open orders.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_all_open_orders() {
    let exchange = init().await;
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner time.")
        .get_all_open_orders()
        .await
        .expect("Couldn't get all open orders.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_all_orders() {
    let exchange = init().await;
    let params = AllOrderReq {
        paginator: None,
        symbol: String::from("BNBBTC"),
    };
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner time.")
        .get_all_orders(&params)
        .await
        .expect("Couldn't get all orders.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_order() {
    let exchange = init().await;
    let pair = exchange
        .get_pair("BNBBTC")
        .await
        .expect("Couldn't get pair handle.")
        .read()
        .expect("Couldn't read pair.");
    let transaction = exchange
        .inner_client()
        .expect("Couldn't get inner time.")
        .limit_sell(
            pair,
            Decimal::new(1, 1),
            Decimal::new(2, 3),
            TimeInForce::GTC,
            false,
        )
        .await
        .expect("Couldn't limit sell.");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner time.")
        .get_order("BNBBTC", transaction.order_id)
        .await
        .expect("Couldn't get order.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_buy() {
    let exchange = init().await;
    let pair = exchange
        .get_pair("BNBBTC")
        .await
        .expect("Couldn't get pair handle.")
        .read()
        .expect("Couldn't read pair.");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner time.")
        .limit_buy(
            pair,
            Decimal::new(1, 1),
            Decimal::new(17, 4),
            TimeInForce::GTC,
            false,
        )
        .await
        .expect("Couldn't limit buy.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn rounded_limit_buy() {
    let exchange = init().await;
    let pair = exchange
        .get_pair("BNBBTC")
        .await
        .expect("Couldn't get pair handle.")
        .read()
        .expect("Couldn't read pair.");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner time.")
        .limit_buy(
            pair,
            Decimal::new(12345678, 8),
            Decimal::new(1, 3),
            TimeInForce::GTC,
            false,
        )
        .await
        .expect("Couldn't limit buy.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_sell() {
    let exchange = init().await;
    let pair = exchange
        .get_pair("BNBBTC")
        .await
        .expect("Couldn't get pair handle.")
        .read()
        .expect("Couldn't read pair.");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner time.")
        .limit_sell(
            pair,
            Decimal::new(1, 1),
            Decimal::new(2, 3),
            TimeInForce::GTC,
            false,
        )
        .await
        .expect("Couldn't limit sell.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_sell_fok() {
    let exchange = init().await;
    let pair = exchange
        .get_pair("BNBBTC")
        .await
        .expect("Couldn't get pair handle.")
        .read()
        .expect("Couldn't read pair.");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner time.")
        .limit_sell(
            pair,
            Decimal::new(1, 1),
            Decimal::new(2, 3),
            TimeInForce::FOK,
            false,
        )
        .await
        .expect("Couldn't limit sell.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_sell_ioc() {
    let exchange = init().await;
    let pair = exchange
        .get_pair("BNBBTC")
        .await
        .expect("Couldn't get pair handle.")
        .read()
        .expect("Couldn't read pair.");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner time.")
        .limit_sell(
            pair,
            Decimal::new(1, 1),
            Decimal::new(2, 3),
            TimeInForce::IOC,
            false,
        )
        .await
        .expect("Couldn't limit sell.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_buy() {
    let exchange = init().await;
    let pair = exchange
        .get_pair("BNBBTC")
        .await
        .expect("Couldn't get pair handle.")
        .read()
        .expect("Couldn't read pair.");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner time.")
        .market_buy(pair, Decimal::new(1, 0))
        .await
        .expect("Couldn't market buy.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_sell() {
    let exchange = init().await;
    let pair = exchange
        .get_pair("BNBBTC")
        .await
        .expect("Couldn't get pair handle.")
        .read()
        .expect("Couldn't read pair.");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner time.")
        .market_sell(pair, Decimal::new(1, 0))
        .await
        .expect("Couldn't market sell.");

    println!("{:?}", resp);
}

#[tokio::test]
async fn cancel_order() {
    let exchange = init().await;
    let pair = exchange
        .get_pair("BNBBTC")
        .await
        .expect("Couldn't get pair handle.")
        .read()
        .expect("Couldn't read pair.");
    let transaction = exchange
        .inner_client()
        .expect("Couldn't get inner time.")
        .limit_sell(
            pair,
            Decimal::new(1, 1),
            Decimal::new(2, 3),
            TimeInForce::GTC,
            false,
        )
        .await
        .expect("Couldn't limit sell.");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner time.")
        .cancel_order("BNBBTC", transaction.order_id)
        .await
        .expect("Couldn't cancel order.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn trade_history() {
    let exchange = init().await;
    let params = TradeHistoryReq {
        paginator: None,
        symbol: String::from("BNBBTC"),
    };

    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner time.")
        .trade_history(&params)
        .await
        .expect("Couldn't trade history.");
    println!("{:?}", resp);
}

async fn init() -> Binance {
    dotenv().ok();
    Binance::new(BinanceParameters {
        sandbox: true,
        credentials: Some(BinanceCredentials {
            api_key: env::var("BINANCE_API_KEY").expect("Couldn't get environment variable"),
            api_secret: env::var("BINANCE_API_SECRET").expect("Couldn't get environment variable"),
        }),
    })
    .await
    .expect("Failed to create Client")
}
