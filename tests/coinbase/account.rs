use dotenv::dotenv;
use openlimits::coinbase::Coinbase;
use std::env;

#[tokio::test]
async fn get_account() {
    let exchange = init();
    let resp = exchange.get_account().await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_open_orders() {
    let exchange = init();
    let resp = exchange.get_open_orders().await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn order_status() {
    let exchange = init();
    let order = exchange.market_buy("BTC-USD", 0.01).await.unwrap();

    let resp = exchange.order_status(order.id).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_buy() {
    let exchange = init();
    let resp = exchange.limit_buy("BTC-USD", 0.01, 9000.0).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_sell() {
    let exchange = init();
    let resp = exchange.limit_sell("BTC-USD", 0.01, 10000.0).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_buy() {
    let exchange = init();
    let resp = exchange.market_buy("BTC-USD", 0.01).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_sell() {
    let exchange = init();
    let resp = exchange.market_sell("BTC-USD", 0.01).await.unwrap();
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
