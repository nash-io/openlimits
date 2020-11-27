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
        .expect("Couldn't get inner client.")
        .get_account(None)
        .await
        .expect("Couldn't get account.");
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
        .expect("Couldn't get inner client.")
        .get_orders(Some(&params))
        .await
        .expect("Couldn't get orders.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_all_orders() {
    let exchange = init().await;
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .get_orders(None)
        .await
        .expect("Couldn't get orders.");
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
        .expect("Couldn't get inner client.")
        .get_orders(Some(&params))
        .await
        .expect("Couldn't get orders.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_order() {
    let exchange = init().await;
    let pair = exchange
        .get_pair("BTC-USD")
        .await
        .expect("Couldn't get pair handle.")
        .read()
        .expect("Couldn't read pair.");
    let order = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .market_buy(pair, Decimal::new(1, 3))
        .await
        .expect("Couldn't market buy.");

    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .get_order(order.id)
        .await
        .expect("Couldn't get order.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_buy() {
    let exchange = init().await;
    let pair = exchange
        .get_pair("BTC-USD")
        .await
        .expect("Couldn't get pair handle.")
        .read()
        .expect("Couldn't read pair.");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .limit_buy(
            pair,
            Decimal::new(1, 3),
            Decimal::new(1000, 0),
            OrderTimeInForce::GTC,
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
        .get_pair("BTC-USD")
        .await
        .expect("Couldn't get pair handle.")
        .read()
        .expect("Couldn't read pair.");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .limit_sell(
            pair,
            Decimal::new(1, 3),
            Decimal::new(1000, 0),
            OrderTimeInForce::GTC,
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
        .get_pair("BTC-USD")
        .await
        .expect("Couldn't get pair handle.")
        .read()
        .expect("Couldn't read pair.");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .limit_sell(
            pair,
            Decimal::new(1, 3),
            Decimal::new(1000, 0),
            OrderTimeInForce::FOK,
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
        .get_pair("BTC-USD")
        .await
        .expect("Couldn't get pair handle.")
        .read()
        .expect("Couldn't read pair.");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .limit_sell(
            pair,
            Decimal::new(1, 3),
            Decimal::new(1000, 0),
            OrderTimeInForce::IOC,
            false,
        )
        .await
        .expect("Couldn't limit sell.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_sell_gtt() {
    let exchange = init().await;
    let pair = exchange
        .get_pair("BTC-USD")
        .await
        .expect("Couldn't get pair handle.")
        .read()
        .expect("Couldn't read pair.");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
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
        .expect("Couldn't limit sell.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_buy() {
    let exchange = init().await;
    let pair = exchange
        .get_pair("BTC-USD")
        .await
        .expect("Couldn't get pair handle.")
        .read()
        .expect("Couldn't read pair.");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .market_buy(pair, Decimal::new(1, 3))
        .await
        .expect("Couldn't market buy.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_sell() {
    let exchange = init().await;
    let pair = exchange
        .get_pair("BTC-USD")
        .await
        .expect("Couldn't get pair handle.")
        .read()
        .expect("Couldn't read pair.");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .market_sell(pair, Decimal::new(1, 3))
        .await
        .expect("Couldn't market sell.");
    println!("{:?}", resp);
}

#[tokio::test]
async fn cancel_all_orders() {
    let exchange = init().await;
    let pair = exchange
        .get_pair("BTC-USD")
        .await
        .expect("Couldn't get pair handle.")
        .read()
        .expect("Couldn't read pair.");
    exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .limit_sell(
            pair.clone(),
            Decimal::new(1, 3),
            Decimal::new(1000, 0),
            OrderTimeInForce::GTC,
            false,
        )
        .await
        .expect("Couldn't limit sell.");
    exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .limit_sell(
            pair.clone(),
            Decimal::new(1, 3),
            Decimal::new(1000, 0),
            OrderTimeInForce::GTC,
            false,
        )
        .await
        .expect("Couldn't limit sell.");

    exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .limit_buy(
            pair,
            Decimal::new(2, 2),
            Decimal::new(2, 2),
            OrderTimeInForce::GTC,
            false,
        )
        .await
        .expect("Couldn't limit buy.");

    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .cancel_all_orders(Some("BTC-USD"))
        .await
        .expect("Couldn't cancel all orders.");

    println!("{:?}", resp);

    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .cancel_all_orders(None)
        .await
        .expect("Couldn't cancel all orders.");

    println!("{:?}", resp);
}

#[tokio::test]
async fn cancel_order() {
    let exchange = init().await;
    let pair = exchange
        .get_pair("BTC-USD")
        .await
        .expect("Couldn't get pair handle.")
        .read()
        .expect("Couldn't read handle.");
    let order = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .limit_sell(
            pair,
            Decimal::new(1, 3),
            Decimal::new(100000, 0),
            OrderTimeInForce::GTC,
            false,
        )
        .await
        .expect("Couldn't limit sell.");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .cancel_order(order.id, Some("BTC-USD"))
        .await
        .expect("Couldn't cancel order.");

    println!("{:?}", resp);
}

#[tokio::test]
async fn get_fills_for_order() {
    let exchange = init().await;
    let pair = exchange
        .get_pair("BTC-USD")
        .await
        .expect("Couldn't get pair handle.")
        .read()
        .expect("Couldn't read pair.");
    let order = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .market_sell(pair, Decimal::new(1, 3))
        .await
        .expect("Couldn't market sell.");

    let params = GetFillsReq {
        order_id: Some(order.id),
        product_id: None,
        paginator: None,
    };

    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .get_fills(Some(&params))
        .await
        .expect("Couldn't get fills.");
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
        .expect("Couldn't get inner client.")
        .get_fills(Some(&params))
        .await
        .expect("Couldn't get fills.");
    println!("{:?}", resp);
}

async fn init() -> Coinbase {
    dotenv().ok();
    Coinbase::new(CoinbaseParameters {
        sandbox: true,
        credentials: Some(CoinbaseCredentials {
            api_key: env::var("COINBASE_API_KEY").expect("Couldn't get environment varilable."),
            api_secret: env::var("COINBASE_API_SECRET")
                .expect("Couldn't get environment varilable."),
            passphrase: env::var("COINBASE_PASSPHRASE")
                .expect("Couldn't get environment varilable."),
        }),
    })
    .await
}
