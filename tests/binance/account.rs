use dotenv::dotenv;
use openlimits::binance::Binance;
use std::env;

#[tokio::test]
async fn get_account() {
    let exchange = init();
    let resp = exchange.get_account().await.unwrap();
    print!("{:?}", resp);
}

#[tokio::test]
async fn get_balance() {
    let exchange = init();
    let resp = exchange.get_balance("BTC").await.unwrap();
    print!("{:?}", resp);
}

#[tokio::test]
async fn get_open_orders() {
    let exchange = init();
    let resp = exchange.get_open_orders("BTCUSDT").await.unwrap();
    print!("{:?}", resp);
}

#[tokio::test]
async fn get_all_open_orders() {
    let exchange = init();
    let resp = exchange.get_all_open_orders().await.unwrap();
    print!("{:?}", resp);
}

fn init() -> Binance {
    dotenv().ok();
    Binance::with_credential(
        &env::var("BINANCE_API_KEY").unwrap(),
        &env::var("BINANCE_API_SECRET").unwrap(),
    )
}
