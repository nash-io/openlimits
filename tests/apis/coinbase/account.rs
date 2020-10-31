use dotenv::dotenv;
use rust_decimal::prelude::Decimal;
use std::env;

use openlimits::{
    coinbase::{
        model::{CancelAfter, GetFillsReq, GetOrderRequest, OrderTimeInForce},
        Coinbase, CoinbaseCredentials, CoinbaseParameters,
    },
    exchange::Exchange,
    exchange_info::ExchangeInfoRetrieval,
};

#[tokio::test]
async fn get_account() {
    let exchange = init().await;
    let resp = exchange
        .inner_client()
        .unwrap()
        .get_account(None)
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_all_open_orders() {
    let exchange = init().await;
    let params = GetOrderRequest {
        status: Some(String::from("open")),
        paginator: None,
        product_id: None,
    };
    let resp = exchange
        .inner_client()
        .unwrap()
        .get_orders(Some(&params))
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_all_orders() {
    let exchange = init().await;
    let resp = exchange
        .inner_client()
        .unwrap()
        .get_orders(None)
        .await
        .unwrap();
    println!("{:?}", resp);

    // let params = GetOrderRequest{
    //     status: Some(String::from("open")),
    //     paginator: None,
    //     product_id: None
    // };
}

#[tokio::test]
async fn get_all_orders_for_a_given_product() {
    let exchange = init().await;

    let params = GetOrderRequest {
        status: None,
        paginator: None,
        product_id: Some(String::from("ETH-BTC")),
    };

    let resp = exchange
        .inner_client()
        .unwrap()
        .get_orders(Some(&params))
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_order() {
    let exchange = init().await;
    let pair = exchange.get_pair("BTC-USD").await.unwrap().read().unwrap();
    let order = exchange
        .inner_client()
        .unwrap()
        .market_buy(pair, Decimal::new(1, 3))
        .await
        .unwrap();

    let resp = exchange
        .inner_client()
        .unwrap()
        .get_order(order.id)
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_buy() {
    let exchange = init().await;
    let pair = exchange.get_pair("BTC-USD").await.unwrap().read().unwrap();
    let resp = exchange
        .inner_client()
        .unwrap()
        .limit_buy(
            pair,
            Decimal::new(1, 3),
            Decimal::new(1000, 0),
            OrderTimeInForce::GTC,
            false,
        )
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_sell() {
    let exchange = init().await;
    let pair = exchange.get_pair("BTC-USD").await.unwrap().read().unwrap();
    let resp = exchange
        .inner_client()
        .unwrap()
        .limit_sell(
            pair,
            Decimal::new(1, 3),
            Decimal::new(1000, 0),
            OrderTimeInForce::GTC,
            false,
        )
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_sell_fok() {
    let exchange = init().await;
    let pair = exchange.get_pair("BTC-USD").await.unwrap().read().unwrap();
    let resp = exchange
        .inner_client()
        .unwrap()
        .limit_sell(
            pair,
            Decimal::new(1, 3),
            Decimal::new(1000, 0),
            OrderTimeInForce::FOK,
            false,
        )
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_sell_ioc() {
    let exchange = init().await;
    let pair = exchange.get_pair("BTC-USD").await.unwrap().read().unwrap();
    let resp = exchange
        .inner_client()
        .unwrap()
        .limit_sell(
            pair,
            Decimal::new(1, 3),
            Decimal::new(1000, 0),
            OrderTimeInForce::IOC,
            false,
        )
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_sell_gtt() {
    let exchange = init().await;
    let pair = exchange.get_pair("BTC-USD").await.unwrap().read().unwrap();
    let resp = exchange
        .inner_client()
        .unwrap()
        .limit_sell(
            pair,
            Decimal::new(1, 3),
            Decimal::new(1000, 0),
            OrderTimeInForce::GTT {
                cancel_after: CancelAfter::Day,
            },
            false,
        )
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_buy() {
    let exchange = init().await;
    let pair = exchange.get_pair("BTC-USD").await.unwrap().read().unwrap();
    let resp = exchange
        .inner_client()
        .unwrap()
        .market_buy(pair, Decimal::new(1, 3))
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_sell() {
    let exchange = init().await;
    let pair = exchange.get_pair("BTC-USD").await.unwrap().read().unwrap();
    let resp = exchange
        .inner_client()
        .unwrap()
        .market_sell(pair, Decimal::new(1, 3))
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn cancel_all_orders() {
    let exchange = init().await;
    let pair = exchange.get_pair("BTC-USD").await.unwrap().read().unwrap();
    exchange
        .inner_client()
        .unwrap()
        .limit_sell(
            pair.clone(),
            Decimal::new(1, 3),
            Decimal::new(1000, 0),
            OrderTimeInForce::GTC,
            false,
        )
        .await
        .unwrap();
    exchange
        .inner_client()
        .unwrap()
        .limit_sell(
            pair.clone(),
            Decimal::new(1, 3),
            Decimal::new(1000, 0),
            OrderTimeInForce::GTC,
            false,
        )
        .await
        .unwrap();

    exchange
        .inner_client()
        .unwrap()
        .limit_buy(
            pair,
            Decimal::new(2, 2),
            Decimal::new(2, 2),
            OrderTimeInForce::GTC,
            false,
        )
        .await
        .unwrap();

    let resp = exchange
        .inner_client()
        .unwrap()
        .cancel_all_orders(Some("BTC-USD"))
        .await
        .unwrap();

    println!("{:?}", resp);

    let resp = exchange
        .inner_client()
        .unwrap()
        .cancel_all_orders(None)
        .await
        .unwrap();

    println!("{:?}", resp);
}

#[tokio::test]
async fn cancel_order() {
    let exchange = init().await;
    let pair = exchange.get_pair("BTC-USD").await.unwrap().read().unwrap();
    let order = exchange
        .inner_client()
        .unwrap()
        .limit_sell(
            pair,
            Decimal::new(1, 3),
            Decimal::new(100000, 0),
            OrderTimeInForce::GTC,
            false,
        )
        .await
        .unwrap();
    let resp = exchange
        .inner_client()
        .unwrap()
        .cancel_order(order.id, Some("BTC-USD"))
        .await
        .unwrap();

    println!("{:?}", resp);
}

#[tokio::test]
async fn get_fills_for_order() {
    let exchange = init().await;
    let pair = exchange.get_pair("BTC-USD").await.unwrap().read().unwrap();
    let order = exchange
        .inner_client()
        .unwrap()
        .market_sell(pair, Decimal::new(1, 3))
        .await
        .unwrap();

    let params = GetFillsReq {
        order_id: Some(order.id),
        product_id: None,
        paginator: None,
    };

    let resp = exchange
        .inner_client()
        .unwrap()
        .get_fills(Some(&params))
        .await
        .unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_fills_for_product() {
    let exchange = init().await;

    let params = GetFillsReq {
        order_id: None,
        product_id: Some(String::from("BTC-USD")),
        paginator: None,
    };

    let resp = exchange
        .inner_client()
        .unwrap()
        .get_fills(Some(&params))
        .await
        .unwrap();
    println!("{:?}", resp);
}

async fn init() -> Coinbase {
    dotenv().ok();
    Coinbase::new(CoinbaseParameters {
        sandbox: true,
        credentials: Some(CoinbaseCredentials {
            api_key: env::var("COINBASE_API_KEY").unwrap(),
            api_secret: env::var("COINBASE_API_SECRET").unwrap(),
            passphrase: env::var("COINBASE_PASSPHRASE").unwrap(),
        }),
    })
    .await
}
