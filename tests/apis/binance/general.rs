use openlimits::{
    binance::{Binance, BinanceParameters},
    exchange::Exchange,
};

#[tokio::test]
async fn ping() {
    let exchange = Binance::new(BinanceParameters::sandbox()).await.expect("Failed to create Client");
    assert_eq!(
        "pong",
        exchange
            .inner_client()
            .expect("Couldn't get inner client.")
            .ping()
            .await
            .expect("Couldn't ping.")
    );
}

#[tokio::test]
async fn get_server_time() {
    let exchange = Binance::new(BinanceParameters::sandbox()).await.expect("Failed to create Client");
    exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .get_server_time()
        .await
        .expect("Couldn't get server time.");
}

#[tokio::test]
async fn get_exchange_info() {
    let exchange = Binance::new(BinanceParameters::sandbox()).await.expect("Failed to create Client");
    let resp = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .get_exchange_info()
        .await
        .expect("Couldn't get exchange info.");
    println!("{:?}", resp);
}
