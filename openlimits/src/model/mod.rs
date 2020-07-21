use chrono::{DateTime, Utc};
use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use rust_decimal::prelude::Decimal;

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
    pub created_at: DateTime<Utc>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Side {
    Buy,
    Sell,
}
