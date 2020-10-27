use chrono::naive::NaiveDateTime;
use openlimits::{
    coinbase::{
        model::{BookRecordL1, CandleRequestParams, DateRange, Paginator},
        Coinbase, CoinbaseParameters,
    },
    exchange::Exchange,
};

#[tokio::test]
async fn products() {
    let exchange = Coinbase::new(CoinbaseParameters::sandbox()).await;
    let res = exchange.inner_client().products().await.unwrap();
    println!("{:?}", res);
}

#[tokio::test]
async fn product() {
    let exchange = Coinbase::new(CoinbaseParameters::sandbox()).await;
    let res = exchange.inner_client().product("BTC-USD").await.unwrap();
    println!("{:?}", res);
}

#[tokio::test]
async fn trades() {
    let exchange = Coinbase::new(CoinbaseParameters::sandbox()).await;
    let res = exchange
        .inner_client()
        .trades("BTC-USD", None)
        .await
        .unwrap();
    println!("{:?}", res);

    let trade = res.last().unwrap();

    let res = exchange
        .inner_client()
        .trades(
            "BTC-USD",
            Some(&Paginator {
                after: Some(trade.trade_id),
                limit: Some(10),
                before: None,
            }),
        )
        .await
        .unwrap();
    println!("{:?}", res);
}

#[tokio::test]
async fn book() {
    let exchange = Coinbase::new(CoinbaseParameters::sandbox()).await;
    let res = exchange
        .inner_client()
        .book::<BookRecordL1>("BTC-USD")
        .await
        .unwrap();
    println!("{:?}", res);
}

#[tokio::test]
async fn ticker() {
    let exchange = Coinbase::new(CoinbaseParameters::sandbox()).await;
    let res = exchange.inner_client().ticker("BTC-USD").await.unwrap();
    println!("{:?}", res);
}

#[tokio::test]
async fn candles() {
    let exchange = Coinbase::new(CoinbaseParameters::sandbox()).await;
    let res = exchange
        .inner_client()
        .candles("BTC-USD", None)
        .await
        .unwrap();
    println!("{:?}", res);

    let res = exchange
        .inner_client()
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
        NaiveDateTime::parse_from_str("2020-08-20T00:00:00.642366Z", "%Y-%m-%dT%H:%M:%S.%fZ")
            .unwrap();

    let res = exchange
        .inner_client()
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
