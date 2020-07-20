use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Constructor, Debug, Default, Deserialize, Serialize)]
pub struct OrderBookRequest {
    pub symbol: String,
}

#[derive(Clone, Constructor, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderBookResponse {
    pub last_update_id: Option<u64>,
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
}

#[derive(Clone, Constructor, Debug, Default, Deserialize, Serialize)]
pub struct Bids {
    pub price: f64,
    pub qty: f64,
}

#[derive(Clone, Constructor, Debug, Default, Deserialize, Serialize)]
pub struct Asks {
    pub price: f64,
    pub qty: f64,
}

#[derive(Clone, Constructor, Debug, Default, Deserialize, Serialize)]
pub struct OpenLimitOrderRequest {
    pub symbol: String,
    pub size: f64,
    pub price: f64,
}

pub struct Order<T> {
    pub id: T,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    Buy,
    Sell,
}
