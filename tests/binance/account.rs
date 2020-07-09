use dotenv::dotenv;
use openlimits::binance::Binance;
use std::env;

#[tokio::test]
async fn get_account() {
    let exchange = init();
    let resp = exchange.get_account().await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_balance() {
    let exchange = init();
    let resp = exchange.get_balance("BTC").await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_open_orders() {
    let exchange = init();
    let resp = exchange.get_open_orders("BNBBTC").await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_all_open_orders() {
    let exchange = init();
    let resp = exchange.get_all_open_orders().await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn order_status() {
    let exchange = init();
    let resp = exchange.order_status("BNBBTC", 411450260).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_buy() {
    let exchange = init();
    let resp = exchange.limit_buy("BNBBTC", 0.1, 0.001).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn limit_sell() {
    let exchange = init();
    let resp = exchange.limit_sell("BNBBTC", 0.1, 0.002).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_buy() {
    let exchange = init();
    let resp = exchange.market_buy("BNBBTC", 0.1).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn market_sell() {
    let exchange = init();
    let resp = exchange.market_sell("BNBBTC", 0.1).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn cancel_order() {
    let exchange = init();
    let resp = exchange.cancel_order("BNBBTC", 411450260).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn trade_history() {
    let exchange = init();
    let resp = exchange.trade_history("BNBBTC").await.unwrap();
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
