use openlimits::coinbase::Coinbase;

#[tokio::test]
async fn products() {
    let exchange = Coinbase::new();
    let res = exchange.products().await.unwrap();
    println!("{:?}", res);
}

#[tokio::test]
async fn trades() {
    let exchange = Coinbase::new();
    let res = exchange.trades("BTC-USD").await.unwrap();
    println!("{:?}", res);
}
