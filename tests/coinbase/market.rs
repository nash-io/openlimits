use openlimits::{
    coinbase::Coinbase,
    exchange::OpenLimits,
    model::{GetHistoricRatesRequest, GetPriceTickerRequest, Interval, OrderBookRequest},
};

#[tokio::test]
async fn order_book() {
    let mut exchange = init().await;
    let req = OrderBookRequest {
        market_pair: "ETH-BTC".to_string(),
    };
    let resp = exchange.order_book(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_price_ticker() {
    let exchange = init().await;
    let req = GetPriceTickerRequest {
        market_pair: "ETH-BTC".to_string(),
    };
    let resp = exchange.get_price_ticker(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_historic_rates() {
    let exchange = init().await;
    let req = GetHistoricRatesRequest {
        market_pair: "ETH-BTC".to_string(),
        interval: Interval::OneHour,
        paginator: None,
    };
    let resp = exchange.get_historic_rates(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_historic_rates_invalid_interval() {
    let exchange = init().await;
    let req = GetHistoricRatesRequest {
        market_pair: "ETH-BTC".to_string(),
        interval: Interval::TwoHours,
        paginator: None,
    };
    let resp = exchange.get_historic_rates(&req).await;
    assert!(resp.is_err());
}

async fn init() -> OpenLimits<Coinbase> {
    OpenLimits {
        exchange: Coinbase::new(true).await,
    }
}
