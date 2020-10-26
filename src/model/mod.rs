use chrono::Duration;
use derive_more::Constructor;
use rust_decimal::prelude::Decimal;
use serde::{Deserialize, Serialize};

use crate::exchange::ExchangeSpec;

#[cfg(feature = "python")]
pub mod python;

pub mod websocket;

#[derive(Serialize, Deserialize, Clone, Constructor, Debug, Default, PartialEq)]
pub struct OrderBookRequest {
    pub market_pair: String,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug, Default, PartialEq)]
pub struct OrderBookResponse {
    pub last_update_id: Option<u64>,
    pub bids: Vec<AskBid>,
    pub asks: Vec<AskBid>,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug, Default, PartialEq)]
pub struct AskBid {
    pub price: Decimal,
    pub qty: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug, Default, PartialEq)]
pub struct OpenLimitOrderRequest {
    pub market_pair: String,
    pub size: Decimal,
    pub price: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug, Default, PartialEq)]
pub struct OpenMarketOrderRequest {
    pub market_pair: String,
    pub size: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct Order<E: ExchangeSpec> {
    pub id: <E as ExchangeSpec>::OrderId,
    pub market_pair: String,
    pub client_order_id: Option<String>,
    pub created_at: Option<u64>,
    pub order_type: OrderType,
    pub side: Side,
    pub status: OrderStatus,
    pub size: Decimal,
    pub price: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct GetOrderRequest<E: ExchangeSpec> {
    pub id: <E as ExchangeSpec>::OrderId,
    pub market_pair: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct Transaction<T> {
    pub id: T,
    pub market_pair: String,
    pub client_order_id: Option<String>,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct CancelOrderRequest<E: ExchangeSpec> {
    pub id: <E as ExchangeSpec>::OrderId,
    pub market_pair: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug, PartialEq)]
pub struct CancelAllOrdersRequest {
    pub market_pair: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct GetOrderHistoryRequest<E: ExchangeSpec> {
    pub market_pair: Option<String>,
    pub paginator: Option<Paginator<E>>,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct OrderCanceled<E: ExchangeSpec> {
    pub id: <E as ExchangeSpec>::OrderId,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct Trade<E: ExchangeSpec> {
    pub id: <E as ExchangeSpec>::TradeId,
    pub order_id: <E as ExchangeSpec>::OrderId,
    pub market_pair: String,
    pub price: Decimal,
    pub qty: Decimal,
    pub fees: Option<Decimal>,
    pub side: Side,
    pub liquidity: Option<Liquidity>,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Liquidity {
    Maker,
    Taker,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TradeHistoryRequest<E: ExchangeSpec> {
    pub market_pair: Option<String>,
    pub order_id: Option<<E as ExchangeSpec>::OrderId>,
    pub paginator: Option<Paginator<E>>,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug, PartialEq)]
pub struct Balance {
    pub asset: String,
    pub total: Decimal,
    pub free: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug, PartialEq)]
pub struct Ticker {
    pub price: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug, PartialEq)]
pub struct Candle {
    pub time: u64,
    pub low: Decimal,
    pub high: Decimal,
    pub open: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug, Default, PartialEq)]
pub struct GetPriceTickerRequest {
    pub market_pair: String,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct GetHistoricRatesRequest<E: ExchangeSpec> {
    pub market_pair: String,
    pub paginator: Option<Paginator<E>>,
    pub interval: Interval,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct GetHistoricTradesRequest<S: ExchangeSpec> {
    pub market_pair: String,
    pub paginator: Option<Paginator<S>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Interval {
    #[serde(rename = "1m")]
    OneMinute,
    #[serde(rename = "3m")]
    ThreeMinutes,
    #[serde(rename = "5m")]
    FiveMinutes,
    #[serde(rename = "15m")]
    FifteenMinutes,
    #[serde(rename = "30m")]
    ThirtyMinutes,
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "2h")]
    TwoHours,
    #[serde(rename = "4h")]
    FourHours,
    #[serde(rename = "6h")]
    SixHours,
    #[serde(rename = "8h")]
    EightHours,
    #[serde(rename = "12h")]
    TwelveHours,
    #[serde(rename = "1d")]
    OneDay,
    #[serde(rename = "3d")]
    ThreeDays,
    #[serde(rename = "1w")]
    OneWeek,
    #[serde(rename = "1mo")]
    OneMonth,
}
impl Into<Duration> for Interval {
    fn into(self) -> Duration {
        match self {
            Self::OneMinute => Duration::minutes(1),
            Self::ThreeMinutes => Duration::minutes(3),
            Self::FiveMinutes => Duration::minutes(5),
            Self::FifteenMinutes => Duration::minutes(15),
            Self::ThirtyMinutes => Duration::minutes(30),
            Self::OneHour => Duration::hours(1),
            Self::TwoHours => Duration::hours(2),
            Self::FourHours => Duration::hours(4),
            Self::SixHours => Duration::hours(6),
            Self::EightHours => Duration::hours(8),
            Self::TwelveHours => Duration::hours(12),
            Self::OneDay => Duration::days(1),
            Self::ThreeDays => Duration::days(3),
            Self::OneWeek => Duration::weeks(1),
            Self::OneMonth => Duration::days(30),
        }
    }
}
impl Interval {
    pub fn to_duration(self) -> Duration {
        self.into()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    PendingCancel,
    Rejected,
    Expired,
    Open,
    Pending,
    Active,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Paginator<E: ExchangeSpec> {
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u64>,
    pub before: Option<<E as ExchangeSpec>::Pagination>,
    pub after: Option<<E as ExchangeSpec>::Pagination>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    StopLimit,
    StopMarket,
    Unknown,
}
