use std::str::FromStr;

use openlimits::{
    prelude::*,
    model::{
        CancelAllOrdersRequest, CancelOrderRequest, GetOrderHistoryRequest, OpenLimitOrderRequest,
        OpenMarketOrderRequest, TimeInForce, TradeHistoryRequest, GetPriceTickerRequest
    },
};
use rust_decimal::prelude::*;
use openlimits::exchange::model::market_pair::MarketPair;
use openlimits::model::currency::Currency;


async fn get_current_price(exchange: &impl Exchange, market_pair: &MarketPair, multiplier: f32) -> Decimal {
    let market_pair = market_pair.clone();
    let ticket = exchange
        .get_price_ticker(&GetPriceTickerRequest { market_pair })
        .await
        .unwrap();
    let price = ticket.price.unwrap();
    price * Decimal::from_f32(multiplier).unwrap()
}

pub async fn limit_buy(exchange: &impl Exchange) {
    let market_pair = MarketPair(Currency::ETH, Currency::BTC);
    let price = get_price(exchange, &market_pair).await;
    let req = OpenLimitOrderRequest {
        client_order_id: None,
        price,
        size: Decimal::new(1, 1),
        market_pair,
        post_only: false,
        time_in_force: TimeInForce::GoodTillCancelled,
    };
    let resp = exchange.limit_buy(&req).await.expect("Couldn't limit buy.");
    println!("{:?}", resp);
}

pub async fn limit_sell(exchange: &impl Exchange) {
    let market_pair = MarketPair(Currency::ETH, Currency::BTC);
    let price = get_price(exchange, &market_pair).await;
    let req = OpenLimitOrderRequest {
        client_order_id: None,
        price,
        post_only: false,
        size: Decimal::new(1, 1),
        market_pair,
        time_in_force: TimeInForce::GoodTillCancelled,
    };
    let resp = exchange
        .limit_sell(&req)
        .await
        .expect("Couldn't limit sell.");
    println!("{:?}", resp);
}

pub async fn post_only(exchange: &impl Exchange) {
    let market_pair = MarketPair(Currency::ETH, Currency::BTC);
    let price = get_current_price(exchange, &market_pair, 1.5).await;
    let req = OpenLimitOrderRequest {
        client_order_id: None,
        price,
        size: Decimal::new(1, 1),
        market_pair: market_pair.clone(),
        post_only: true,
        time_in_force: TimeInForce::GoodTillCancelled,
    };
    let resp = exchange
        .limit_sell(&req)
        .await
        .expect("Couldn't limit sell.");
    println!("{:?}", resp);

    let price = get_current_price(exchange, &market_pair, 0.8).await;
    let req = OpenLimitOrderRequest {
        client_order_id: None,
        price,
        size: Decimal::new(1, 1),
        market_pair,
        post_only: true,
        time_in_force: TimeInForce::GoodTillCancelled,
    };
    let resp = exchange
        .limit_buy(&req)
        .await
        .expect("Couldn't limit buy.");

    println!("{:?}", resp);
}

pub async fn market_buy(exchange: &impl Exchange) {
    let market_pair = MarketPair(Currency::ETH, Currency::BTC);
    let req = OpenMarketOrderRequest {
        client_order_id: None,
        size: Decimal::from_str("0.1").unwrap(),
        market_pair,
    };
    let resp = exchange
        .market_buy(&req)
        .await
        .expect("Couldn't market buy.");
    println!("{:?}", resp);
}

pub async fn market_sell(exchange: &impl Exchange) {
    let market_pair = MarketPair(Currency::ETH, Currency::BTC);
    let req = OpenMarketOrderRequest {
        client_order_id: None,
        size: Decimal::new(1, 1),
        market_pair,
    };
    let resp = exchange
        .market_sell(&req)
        .await
        .expect("Couldn't market sell.");
    println!("{:?}", resp);
}

pub async fn cancel_order(exchange: &impl Exchange) {
    let market_pair = MarketPair(Currency::ETH, Currency::BTC);
    let price = get_current_price(exchange, &market_pair, 1.5).await;
    let req = OpenLimitOrderRequest {
        client_order_id: None,
        price,
        size: Decimal::from_str("1.0").unwrap(),
        market_pair,
        post_only: false,
        time_in_force: TimeInForce::GoodTillCancelled,
    };
    let order = exchange
        .limit_sell(&req)
        .await
        .expect("Couldn't limit sell.");

    let req = CancelOrderRequest {
        id: order.id,
        market_pair: Some(order.market_pair),
    };

    let resp = exchange
        .cancel_order(&req)
        .await
        .expect("Couldn't cancel order.");
    println!("{:?}", resp);
}

pub async fn cancel_all_orders(exchange: &impl Exchange) {
    let market_pair = MarketPair(Currency::ETH, Currency::BTC);
    let price = get_current_price(exchange, &market_pair, 1.5).await;
    let req = OpenLimitOrderRequest {
        client_order_id: None,
        price,
        size: Decimal::from_str("1.0").unwrap(),
        market_pair,
        post_only: false,
        time_in_force: TimeInForce::GoodTillCancelled,
    };
    exchange
        .limit_sell(&req)
        .await
        .expect("Couldn't limit sell.");

    exchange
        .limit_sell(&req)
        .await
        .expect("Couldn't limit sell.");

    let market_pair = Some(MarketPair(Currency::ETH, Currency::BTC));
    let req = CancelAllOrdersRequest { market_pair };

    let resp = exchange
        .cancel_all_orders(&req)
        .await
        .expect("Couldn't cancel all orders.");
    println!("{:?}", resp);
}

pub async fn get_order_history(exchange: &impl Exchange) {
    let market_pair = MarketPair(Currency::ETH, Currency::BTC);
    let market_pair = Some(market_pair);
    let req = GetOrderHistoryRequest {
        market_pair,
        order_status: None,
        paginator: None,
    };

    let resp = exchange
        .get_order_history(&req)
        .await
        .expect("Couldn't get order history.");
    println!("{:?}", resp);
}

async fn get_price(exchange: &impl Exchange, market_pair: &MarketPair) -> Decimal {
    let market_pair = market_pair.clone();
    let get_price_ticker_request = GetPriceTickerRequest { market_pair };
    let ticker = exchange.get_price_ticker(&get_price_ticker_request).await.expect("Couldn't get ticker.");
    ticker.price.expect("Couldn't get price.")
}

pub async fn get_all_open_orders(exchange: &impl Exchange) {
    let market_pair = MarketPair(Currency::ETH, Currency::BTC);
    let price = get_price(exchange, &market_pair).await;
    let req = OpenLimitOrderRequest {
        client_order_id: None,
        price,
        size: Decimal::new(1, 1),
        market_pair,
        post_only: false,
        time_in_force: TimeInForce::GoodTillCancelled,
    };
    exchange
        .limit_sell(&req)
        .await
        .expect("Couldn't limit sell.");

    let resp = exchange
        .get_all_open_orders()
        .await
        .expect("Couldn't get all open orders.");
    println!("{:?}", resp);
}

pub async fn get_account_balances(exchange: &impl Exchange) {
    let resp = exchange
        .get_account_balances(None)
        .await
        .expect("Couldn't get acount balances.");
    println!("{:?}", resp);
}

pub async fn get_trade_history(exchange: &impl Exchange) {
    let market_pair = MarketPair(Currency::ETH, Currency::BTC);
    let market_pair = Some(market_pair);
    let req = TradeHistoryRequest { market_pair, ..Default::default() };

    let resp = exchange
        .get_trade_history(&req)
        .await
        .expect("Couldn't get trade history.");
    println!("{:?}", resp);
}