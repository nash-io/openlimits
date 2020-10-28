use openlimits::{
    binance::{Binance, BinanceParameters},
    exchange::Exchange,
};

#[tokio::test]
async fn ping() {
    let exchange = Binance::new(BinanceParameters::sandbox()).await;
    assert_eq!(
        "pong",
        exchange.inner_client().unwrap().ping().await.unwrap()
    );
}

#[tokio::test]
async fn get_server_time() {
    let exchange = Binance::new(BinanceParameters::sandbox()).await;
    exchange
        .inner_client()
        .unwrap()
        .get_server_time()
        .await
        .unwrap();
}

#[tokio::test]
async fn get_exchange_info() {
    let exchange = Binance::new(BinanceParameters::sandbox()).await;
    let resp = exchange
        .inner_client()
        .unwrap()
        .get_exchange_info()
        .await
        .unwrap();
    println!("{:?}", resp);
}
