use derive_more::Constructor;
use rust_decimal::prelude::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Constructor, Debug, Default)]
pub struct OrderBookRequest {
    pub symbol: String,
}

#[derive(Clone, Constructor, Debug, Default)]
pub struct OrderBookResponse {
    pub last_update_id: Option<u64>,
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
}

#[derive(Clone, Constructor, Debug, Default)]
pub struct Bids {
    pub price: Decimal,
    pub qty: Decimal,
}

#[derive(Clone, Constructor, Debug, Default)]
pub struct Asks {
    pub price: Decimal,
    pub qty: Decimal,
}

#[derive(Clone, Constructor, Debug, Default)]
pub struct OpenLimitOrderRequest {
    pub symbol: String,
    pub size: Decimal,
    pub price: Decimal,
}

#[derive(Clone, Constructor, Debug, Default)]
pub struct OpenMarketOrderRequest {
    pub symbol: String,
    pub size: Decimal,
}

#[derive(Clone, Constructor, Debug)]
pub struct Order<T> {
    pub id: T,
    pub symbol: String,
    pub client_order_id: Option<String>,
    pub created_at: u64,
}

#[derive(Clone, Constructor, Debug)]
pub struct CancelOrderRequest<T> {
    pub id: T,
    pub pair: Option<String>,
}

#[derive(Clone, Constructor, Debug)]
pub struct CancelAllOrdersRequest {
    pub pair: Option<String>,
}

#[derive(Clone, Constructor, Debug)]
pub struct OrderCanceled<T> {
    pub id: T,
}

#[derive(Clone, Constructor, Debug)]
pub struct Trade<T, O> {
    pub id: T,
    pub order_id: O,
    pub pair: String,
    pub price: Decimal,
    pub qty: Decimal,
    pub fees: Decimal,
    pub side: Side,
    pub liquidity: Option<Liquidity>,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Liquidity {
    Maker,
    Taker,
}

#[derive(Default)]
pub struct TradeHistoryRequest<T> {
    pub pair: Option<String>,
    pub order_id: Option<T>,
}

#[derive(Clone, Constructor, Debug)]
pub struct Balance {
    pub asset: String,
    pub total: Decimal,
    pub free: Decimal,
}

#[derive(Clone, Constructor, Debug)]
pub struct Ticker {
    pub price: Decimal,
}

#[derive(Clone, Constructor, Debug, Default)]
pub struct GetPriceTickerRequest {
    pub symbol: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Side {
    Buy,
    Sell,
}
