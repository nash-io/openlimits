use openlimits::{
    binance::Binance,
    binance::BinanceParameters,
    exchange::Exchange,
    exchange::{ExchangeMarketData, OpenLimits},
    exchange_info::ExchangeInfoRetrieval,
    model::{GetHistoricRatesRequest, GetPriceTickerRequest, Interval, OrderBookRequest},
};

#[tokio::test]
async fn order_book() {
    let exchange = init().await;
    let req = OrderBookRequest {
        market_pair: "BNBBTC".to_string(),
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
        market_pair: "BNBBTC".to_string(),
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
        market_pair: "BNBBTC".to_string(),
        interval: Interval::OneHour,
        paginator: None,
    };
    let _response = exchange
        .get_historic_rates(&req)
        .await
        .expect("Couldn't get historic rates.");
}

#[tokio::test]
async fn pair() {
    let exchange = Binance::new(BinanceParameters::sandbox()).await.expect("Failed to create Client");
    let _response = exchange
        .get_pair("BTCUSDT")
        .await
        .expect("Couldn't get pair.");
}

async fn init() -> Binance {
    let parameters = BinanceParameters {
        credentials: None,
        sandbox: true,
    };

    OpenLimits::instantiate(parameters).await.expect("Failed to create Client")
}
