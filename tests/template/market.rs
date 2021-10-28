use openlimits::{
    prelude::*,
    model::{GetHistoricRatesRequest, GetPriceTickerRequest, Interval, OrderBookRequest},
};

pub async fn order_book(exchange: &impl Exchange) {
    let req = OrderBookRequest {
        market_pair: "BNBBTC".to_string(),
    };
    let _response = exchange
        .order_book(&req)
        .await
        .expect("Couldn't get order book.");
}

pub async fn get_price_ticker(exchange: &impl Exchange) {
    let req = GetPriceTickerRequest {
        market_pair: "BNBBTC".to_string(),
    };
    let _response = exchange
        .get_price_ticker(&req)
        .await
        .expect("Couldn't get price ticker.");
}

pub async fn get_historic_rates(exchange: &impl Exchange) {
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

pub async fn pair(exchange: &impl Exchange) {
    let _response = exchange
        .get_pair("BTCUSDT")
        .await
        .expect("Couldn't get pair.");
}