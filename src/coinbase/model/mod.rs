use crate::utils::string_or_float_to_float;
use serde::{Deserialize, Serialize};
pub mod websocket;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: String,
    pub display_name: String,
    pub quote_currency: String,
    pub base_currency: String,
    #[serde(with = "string_or_float_to_float")]
    pub base_increment: f64,
    #[serde(with = "string_or_float_to_float")]
    pub quote_increment: f64,
    #[serde(with = "string_or_float_to_float")]
    pub base_min_size: f64,
    #[serde(with = "string_or_float_to_float")]
    pub base_max_size: f64,
    pub min_market_funds: String,
    pub max_market_funds: String,
    pub status: String,
    pub status_message: String,
    pub cancel_only: bool,
    pub limit_only: bool,
    pub post_only: bool,
    pub trading_disabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: String,
    #[serde(with = "string_or_float_to_float")]
    pub balance: f64,
    #[serde(with = "string_or_float_to_float")]
    pub available: f64,
    #[serde(with = "string_or_float_to_float")]
    pub hold: f64,
    pub profile_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Candle {
    pub time: i64,
    pub low: f64,
    pub high: f64,
    pub open: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trade {
    pub trade_id: i64,
    pub time: String,
    pub size: String,
    #[serde(with = "string_or_float_to_float")]
    pub price: f64,
    pub side: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ticker {
    pub trade_id: i64,
    #[serde(with = "string_or_float_to_float")]
    pub price: f64,
    #[serde(with = "string_or_float_to_float")]
    pub size: f64,
    #[serde(with = "string_or_float_to_float")]
    pub bid: f64,
    #[serde(with = "string_or_float_to_float")]
    pub ask: f64,
    #[serde(with = "string_or_float_to_float")]
    pub volume: f64,
    pub time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Book<T> {
    pub sequence: usize,
    pub bids: Vec<T>,
    pub asks: Vec<T>,
}

pub trait BookLevel {
    fn level() -> u8;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookRecordL1 {
    #[serde(with = "string_or_float_to_float")]
    pub price: f64,
    #[serde(with = "string_or_float_to_float")]
    pub size: f64,
    pub num_orders: usize,
}

impl BookLevel for BookRecordL1 {
    fn level() -> u8 {
        1
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookRecordL2 {
    #[serde(with = "string_or_float_to_float")]
    pub price: f64,
    #[serde(with = "string_or_float_to_float")]
    pub size: f64,
    pub num_orders: usize,
}

impl BookLevel for BookRecordL2 {
    fn level() -> u8 {
        2
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookRecordL3 {
    #[serde(with = "string_or_float_to_float")]
    pub price: f64,
    #[serde(with = "string_or_float_to_float")]
    pub size: f64,
    pub order_id: String,
}

impl BookLevel for BookRecordL3 {
    fn level() -> u8 {
        3
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bids {
    #[serde(with = "string_or_float_to_float")]
    pub price: f64,
    #[serde(with = "string_or_float_to_float")]
    pub qty: f64,
    pub num_orders: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asks {
    #[serde(with = "string_or_float_to_float")]
    pub price: f64,
    #[serde(with = "string_or_float_to_float")]
    pub qty: f64,
    pub num_orders: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum OrderSide {
    Buy,
    Sell,
}
