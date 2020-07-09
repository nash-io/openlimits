use openlimits::coinbase::model::BookRecordL1;
use openlimits::coinbase::Coinbase;
#[tokio::test]
async fn products() {
    let exchange = Coinbase::new();
    let res = exchange.products().await.unwrap();
    println!("{:?}", res);
}

#[tokio::test]
async fn product() {
    let exchange = Coinbase::new();
    let res = exchange.product("BTC-USD").await.unwrap();
    println!("{:?}", res);
}

#[tokio::test]
async fn trades() {
    let exchange = Coinbase::new();
    let res = exchange.trades("BTC-USD").await.unwrap();
    println!("{:?}", res);
}

#[tokio::test]
async fn book() {
    let exchange = Coinbase::new();
    let res = exchange.book::<BookRecordL1>("BTC-USD").await.unwrap();
    println!("{:?}", res);
}

#[tokio::test]
async fn ticker() {
    let exchange = Coinbase::new();
    let res = exchange.ticker("BTC-USD").await.unwrap();
    println!("{:?}", res);
}

#[tokio::test]
async fn candles() {
    let exchange = Coinbase::new();
    let res = exchange.candles("BTC-USD").await.unwrap();
    println!("{:?}", res);
}
