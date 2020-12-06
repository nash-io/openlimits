use nash_native_client::ws_client::client::Environment;
use openlimits::{
    exchange::{ExchangeMarketData, OpenLimits},
    exchange_info::ExchangeInfoRetrieval,
    model::GetHistoricTradesRequest,
    model::Paginator,
    model::{GetHistoricRatesRequest, GetPriceTickerRequest, Interval, OrderBookRequest},
    nash::Nash,
    nash::NashCredentials,
    nash::NashParameters,
};

use dotenv::dotenv;
use std::env;

#[tokio::test]
async fn order_book() {
    let exchange = init().await;
    let req = OrderBookRequest {
        market_pair: "eth_btc".to_string(),
    };
    let _response = exchange
        .order_book(&req)
        .await
        .expect("Couldn't get order book.");
}

#[tokio::test]
async fn get_price_ticker() {
    let exchange = init().await;
    let req = GetPriceTickerRequest {
        market_pair: "eth_btc".to_string(),
    };
    let _response = exchange
        .get_price_ticker(&req)
        .await
        .expect("Couldn't get price ticker.");
}

#[tokio::test]
async fn get_historic_rates() {
    let exchange = init().await;
    let req = GetHistoricRatesRequest {
        market_pair: "eth_btc".to_string(),
        interval: Interval::OneHour,
        paginator: None,
    };
    let _response = exchange
        .get_historic_rates(&req)
        .await
        .expect("Couldn't get historic rates.");
}

#[tokio::test]
async fn get_historic_trades() {
    let exchange = init().await;
    let req = GetHistoricTradesRequest {
        market_pair: "eth_btc".to_string(),
        paginator: Some(Paginator {
            limit: Some(100),
            ..Default::default()
        }),
    };
    let _response = exchange
        .get_historic_trades(&req)
        .await
        .expect("Couldn't get historic trades.");
}

#[tokio::test]
async fn retrieve_pairs() {
    let exchange = init().await;
    let _response = exchange
        .refresh_market_info()
        .await
        .expect("Couldn't get pairs.");
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

async fn init() -> Nash {
    dotenv().ok();

    let parameters = NashParameters {
        affiliate_code: None,
        credentials: Some(NashCredentials {
            secret: env::var("NASH_API_SECRET").expect("Couldn't get environment variable."),
            session: env::var("NASH_API_KEY").expect("Couldn't get environment variable."),
        }),
        environment: Environment::Sandbox,
        client_id: 1,
        timeout: 1000,
    };

    OpenLimits::instantiate(parameters)
        .await
        .expect("Failed to create Client")
}
