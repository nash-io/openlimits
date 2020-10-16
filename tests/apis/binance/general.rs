use crate::openlimits::exchange::ExchangeInstantiation;
use openlimits::binance::{Binance, BinanceParameters};

#[tokio::test]
async fn ping() {
    let exchange = Binance::new(BinanceParameters::sandbox()).await;
    assert_eq!("pong", exchange.ping().await.unwrap());
}

#[tokio::test]
async fn get_server_time() {
    let exchange = Binance::new(BinanceParameters::sandbox()).await;
    exchange.get_server_time().await.unwrap();
}

#[tokio::test]
async fn get_exchange_info() {
    let exchange = Binance::new(BinanceParameters::sandbox()).await;
    let resp = exchange.get_exchange_info().await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_pair() {
    let exchange = Binance::new(BinanceParameters::sandbox()).await;
    let res = exchange.get_pair("BNBBTC").unwrap();
    println!("{:?}", res);
}
