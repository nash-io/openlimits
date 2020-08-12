use chrono::naive::NaiveDateTime;
use coinbase::model::BookRecordL1;
use coinbase::model::{CandleRequestParams, DateRange};
use coinbase::Coinbase;

#[tokio::test]
async fn products() {
    let exchange = Coinbase::new(true);
    let res = exchange.products().await.unwrap();
    println!("{:?}", res);
}

#[tokio::test]
async fn product() {
    let exchange = Coinbase::new(true);
    let res = exchange.product("BTC-USD").await.unwrap();
    println!("{:?}", res);
}

#[tokio::test]
async fn trades() {
    let exchange = Coinbase::new(true);
    let res = exchange.trades("BTC-USD").await.unwrap();
    println!("{:?}", res);
}

#[tokio::test]
async fn book() {
    let exchange = Coinbase::new(true);
    let res = exchange.book::<BookRecordL1>("BTC-USD").await.unwrap();
    println!("{:?}", res);
}

#[tokio::test]
async fn ticker() {
    let exchange = Coinbase::new(true);
    let res = exchange.ticker("BTC-USD").await.unwrap();
    println!("{:?}", res);
}

#[tokio::test]
async fn candles() {
    let exchange = Coinbase::new(true);
    let res = exchange.candles("BTC-USD", None).await.unwrap();
    println!("{:?}", res);

    let res = exchange
        .candles(
            "BTC-USD",
            Some(&CandleRequestParams {
                granularity: Some(60),
                daterange: None,
            }),
        )
        .await
        .unwrap();
    println!("{:?}", res);
    let date =
        NaiveDateTime::parse_from_str("2020-01-20T00:00:00.642366Z", "%Y-%m-%dT%H:%M:%S.%fZ")
            .unwrap();

    let res = exchange
        .candles(
            "BTC-USD",
            Some(&CandleRequestParams {
                granularity: Some(3600),
                daterange: Some(DateRange {
                    start: Some(date),
                    end: None,
                }),
            }),
        )
        .await
        .unwrap();
    println!("{:?}", res);
}
