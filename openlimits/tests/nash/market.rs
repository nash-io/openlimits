use openlimits::{
    exchange::OpenLimits,
    exchange_info::ExchangeInfoRetrieval,
    model::{GetHistoricRatesRequest, GetPriceTickerRequest, Interval, OrderBookRequest},
    nash::Nash,
};

use dotenv::dotenv;
use std::env;

#[tokio::test]
async fn order_book() {
    let exchange = init().await;
    let req = OrderBookRequest {
        market_pair: "eth_btc".to_string(),
    };
    let resp = exchange.order_book(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_price_ticker() {
    let exchange = init().await;
    let req = GetPriceTickerRequest {
        market_pair: "eth_btc".to_string(),
    };
    let resp = exchange.get_price_ticker(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_historic_rates() {
    let exchange = init().await;
    let req = GetHistoricRatesRequest {
        market_pair: "eth_btc".to_string(),
        interval: Interval::OneHour,
        paginator: None,
    };
    let resp = exchange.get_historic_rates(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn retrieve_pairs() {
    let exchange = init().await;
    let pairs = exchange.refresh_market_info().await.unwrap();
    println!("{:?}", pairs);
}

// #[tokio::test]
// async fn get_historic_rates_invalid_interval() {
//     let mut exchange = init().await;
//     let req = GetHistoricRatesRequest {
//         market_pair: "eth_btc".to_string(),
//         interval: Interval::TwoHours,
//         paginator: None,
//     };
//     let resp = exchange.get_historic_rates(&req).await;
//     assert!(resp.is_err());
// }

async fn init() -> OpenLimits<Nash> {
    dotenv().ok();

    let exchange = Nash::with_credential(
        &env::var("NASH_API_SECRET").unwrap(),
        &env::var("NASH_API_KEY").unwrap(),
        1234,
        false,
        100000,
    )
    .await;

    OpenLimits { exchange }
}
