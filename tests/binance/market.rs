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
    let resp = exchange.order_book(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_price_ticker() {
    let exchange = init().await;
    let req = GetPriceTickerRequest {
        market_pair: "BNBBTC".to_string(),
    };
    let resp = exchange.get_price_ticker(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn get_historic_rates() {
    let exchange = init().await;
    let req = GetHistoricRatesRequest {
        market_pair: "BNBBTC".to_string(),
        interval: Interval::OneHour,
        paginator: None,
    };
    let resp = exchange.get_historic_rates(&req).await.unwrap();
    println!("{:?}", resp);
}

#[tokio::test]
async fn pair() {
    let exchange = Binance::new(BinanceParameters::sandbox()).await;
    let res = exchange.get_pair("BTCUSDT").await.unwrap();
    println!("{:?}", res);
}

async fn init() -> Binance {
    let parameters = BinanceParameters {
        credentials: None,
        sandbox: true,
    };

    OpenLimits::instantiate(parameters).await
}
