use openlimits::{
    coinbase::Coinbase,
    exchange::Exchange,
    model::{GetHistoricRatesRequest, GetPriceTickerRequest, Interval, OrderBookRequest},
};

#[tokio::test]
async fn order_book() {
    let exchange = Coinbase::new(true);
    let req = OrderBookRequest {
        symbol: "ETH-BTC".to_string(),
    };
    let resp = exchange.order_book(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_price_ticker() {
    let exchange = Coinbase::new(true);
    let req = GetPriceTickerRequest {
        symbol: "ETH-BTC".to_string(),
    };
    let resp = exchange.get_price_ticker(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_historic_rates() {
    let exchange = Coinbase::new(true);
    let req = GetHistoricRatesRequest {
        symbol: "ETH-BTC".to_string(),
        interval: Interval::OneHour,
        paginator: None,
    };
    let resp = exchange.get_historic_rates(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_historic_rates_invalid_interval() {
    let exchange = Coinbase::new(true);
    let req = GetHistoricRatesRequest {
        symbol: "ETH-BTC".to_string(),
        interval: Interval::TwoHours,
        paginator: None,
    };
    let resp = exchange.get_historic_rates(&req).await;
    assert!(resp.is_err());
}
