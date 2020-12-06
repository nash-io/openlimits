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
    let exchange = Coinbase::new(CoinbaseParameters::sandbox())
        .await
        .expect("Failed to create Client");
    let res = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .products()
        .await
        .expect("Couldn't get products.");
    println!("{:?}", res);
}

#[tokio::test]
async fn product() {
    let exchange = Coinbase::new(CoinbaseParameters::sandbox())
        .await
        .expect("Failed to create Client");
    let res = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .product("BTC-USD")
        .await
        .expect("Couldn't get product.");
    println!("{:?}", res);
}

#[tokio::test]
async fn trades() {
    let exchange = Coinbase::new(CoinbaseParameters::sandbox())
        .await
        .expect("Failed to create Client");
    let res = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .trades("BTC-USD", None)
        .await
        .expect("Couldn't get trades.");
    println!("{:?}", res);

    let trade = res.last().expect("Couldn't get last trade.");

    let res = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .trades(
            "BTC-USD",
            Some(&Paginator {
                after: Some(trade.trade_id),
                limit: Some(10),
                before: None,
            }),
        )
        .await
        .expect("Couldn't get trades.");
    println!("{:?}", res);
}

#[tokio::test]
async fn book() {
    let exchange = Coinbase::new(CoinbaseParameters::sandbox())
        .await
        .expect("Failed to create Client");
    let res = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .book::<BookRecordL1>("BTC-USD")
        .await
        .expect("Couldn't get book.");
    println!("{:?}", res);
}

#[tokio::test]
async fn ticker() {
    let exchange = Coinbase::new(CoinbaseParameters::sandbox())
        .await
        .expect("Failed to create Client");
    let res = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .ticker("BTC-USD")
        .await
        .expect("Couldn't get ticker.");
    println!("{:?}", res);
}

#[tokio::test]
async fn candles() {
    let exchange = Coinbase::new(CoinbaseParameters::sandbox())
        .await
        .expect("Failed to create Client");
    let res = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .candles("BTC-USD", None)
        .await
        .expect("Couldn't get candles.");
    println!("{:?}", res);

    let res = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
        .candles(
            "BTC-USD",
            Some(&CandleRequestParams {
                granularity: Some(60),
                daterange: None,
            }),
        )
        .await
        .expect("Couldn't get candles.");
    println!("{:?}", res);
    let date =
        NaiveDateTime::parse_from_str("2020-08-20T00:00:00.642366Z", "%Y-%m-%dT%H:%M:%S.%fZ")
            .expect("Couldn't parse date from string.");

    let res = exchange
        .inner_client()
        .expect("Couldn't get inner client.")
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
        .expect("Couldn't get candles.");
    println!("{:?}", res);
}
