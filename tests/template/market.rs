use openlimits::{
    prelude::*,
    model::{GetHistoricRatesRequest, GetPriceTickerRequest, Interval, OrderBookRequest},
};
use openlimits_exchange::model::market_pair::MarketPair;
use openlimits_exchange::model::currency::Currency;

pub async fn order_book(exchange: &impl Exchange) {
    let market_pair = MarketPair(Currency::ETH, Currency::BTC);
    let req = OrderBookRequest { market_pair };
    let _response = exchange
        .order_book(&req)
        .await
        .expect("Couldn't get order book.");
}

pub async fn get_price_ticker(exchange: &impl Exchange) {
    let market_pair = MarketPair(Currency::ETH, Currency::BTC);
    let req = GetPriceTickerRequest { market_pair };
    let _response = exchange
        .get_price_ticker(&req)
        .await
        .expect("Couldn't get price ticker.");
}

pub async fn get_historic_rates(exchange: &impl Exchange) {
    let market_pair = MarketPair(Currency::ETH, Currency::BTC);
    let req = GetHistoricRatesRequest {
        market_pair,
        interval: Interval::OneHour,
        paginator: None,
    };
    let _response = exchange
        .get_historic_rates(&req)
        .await
        .expect("Couldn't get historic rates.");
}

pub async fn pair(exchange: &impl Exchange) {
    let market_pair = MarketPair(Currency::ETH, Currency::BTC);
    let response = exchange
        .get_pair(&market_pair)
        .await
        .expect("Couldn't get pair.");
    println!("{:#?}", response);
}