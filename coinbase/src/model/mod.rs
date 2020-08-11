use chrono;

use serde::{Deserialize, Serialize};
use shared::{datetime_from_string, string_to_decimal};
pub mod websocket;
use rust_decimal::prelude::Decimal;

pub type DateTime = chrono::DateTime<chrono::Utc>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: String,
    pub display_name: String,
    pub quote_currency: String,
    pub base_currency: String,
    #[serde(with = "string_to_decimal")]
    pub base_increment: Decimal,
    #[serde(with = "string_to_decimal")]
    pub quote_increment: Decimal,
    #[serde(with = "string_to_decimal")]
    pub base_min_size: Decimal,
    #[serde(with = "string_to_decimal")]
    pub base_max_size: Decimal,
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
    pub currency: String,
    #[serde(with = "string_to_decimal")]
    pub balance: Decimal,
    #[serde(with = "string_to_decimal")]
    pub available: Decimal,
    #[serde(with = "string_to_decimal")]
    pub hold: Decimal,
    pub profile_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Candle {
    pub time: u64,
    pub low: Decimal,
    pub high: Decimal,
    pub open: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trade {
    pub trade_id: u64,
    #[serde(with = "datetime_from_string")]
    pub time: DateTime,
    pub size: String,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    pub side: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Fill {
    pub trade_id: u64,
    pub product_id: String,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub size: Decimal,
    pub order_id: String,
    #[serde(with = "datetime_from_string")]
    pub created_at: DateTime,
    pub liquidity: String,
    #[serde(with = "string_to_decimal")]
    pub fee: Decimal,
    pub settled: bool,
    pub side: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ticker {
    pub trade_id: i64,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub size: Decimal,
    #[serde(with = "string_to_decimal")]
    pub bid: Decimal,
    #[serde(with = "string_to_decimal")]
    pub ask: Decimal,
    #[serde(with = "string_to_decimal")]
    pub volume: Decimal,
    #[serde(with = "datetime_from_string")]
    pub time: DateTime,
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
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub size: Decimal,
    pub num_orders: usize,
}

impl BookLevel for BookRecordL1 {
    fn level() -> u8 {
        1
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookRecordL2 {
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub size: Decimal,
    pub num_orders: usize,
}

impl BookLevel for BookRecordL2 {
    fn level() -> u8 {
        2
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookRecordL3 {
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub size: Decimal,
    pub order_id: String,
}

impl BookLevel for BookRecordL3 {
    fn level() -> u8 {
        3
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub id: String,
    pub product_id: String,
    pub side: OrderSide,
    pub stp: Option<String>,
    #[serde(flatten)]
    pub _type: OrderType,
    pub post_only: bool,
    #[serde(with = "datetime_from_string")]
    pub created_at: DateTime,
    #[serde(with = "string_to_decimal")]
    pub fill_fees: Decimal,
    #[serde(with = "string_to_decimal")]
    pub filled_size: Decimal,
    #[serde(with = "string_to_decimal")]
    pub executed_value: Decimal,
    pub status: OrderStatus,
    pub settled: bool,
    #[serde(flatten)]
    pub stop: Option<OrderStop>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderRequest {
    pub side: OrderSide,
    pub client_oid: Option<String>,
    pub product_id: String,
    #[serde(flatten)]
    pub _type: OrderRequestType,
    #[serde(flatten)]
    pub stop: Option<OrderStop>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CancelAllOrders {
    pub product_id: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct CancelOrder {
    pub product_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum OrderType {
    Limit {
        #[serde(with = "string_to_decimal")]
        size: Decimal,
        #[serde(with = "string_to_decimal")]
        price: Decimal,
        #[serde(flatten)]
        time_in_force: OrderTimeInForce,
    },
    Market {
        #[serde(default)]
        #[serde(with = "string_to_decimal")]
        size: Decimal,
        #[serde(default)]
        #[serde(with = "string_to_decimal")]
        funds: Decimal,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum OrderRequestType {
    Limit {
        price: Decimal,
        size: Decimal,
        post_only: bool,
        #[serde(flatten)]
        time_in_force: Option<OrderTimeInForce>,
    },
    Market {
        #[serde(flatten)]
        _type: OrderRequestMarketType,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum OrderRequestMarketType {
    Size { size: Decimal },
    Funds { funds: Decimal },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "time_in_force")]
pub enum OrderTimeInForce {
    GTC,
    GTT { expire_time: String },
    IOC,
    FOK,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum OrderStatus {
    Open,
    Done,
    Pending,
    Active,
    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderStop {
    stop_price: Decimal,
    #[serde(rename = "stop")]
    _type: OrderStopType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum OrderStopType {
    Loss,
    Entry,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Paginator {
    pub before: Option<i64>,
    pub limit: Option<i64>,
    pub after: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DateRange {
    #[serde(with = "datetime_from_string")]
    pub start: DateTime,
    #[serde(with = "datetime_from_string")]
    pub end: DateTime,
}
