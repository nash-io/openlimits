use chrono::Duration;
use derive_more::Constructor;
use rust_decimal::prelude::Decimal;
use serde::{Deserialize, Serialize};

use crate::exchange::ExchangeSpec;
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
pub struct Order<S: ExchangeSpec> {
    pub id: <S as ExchangeSpec>::OrderId,
    pub market_pair: String,
    pub client_order_id: Option<String>,
    pub created_at: Option<u64>,
    pub order_type: String,
    pub side: Side,
    pub status: OrderStatus,
    pub size: Decimal,
    pub price: Option<Decimal>,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct GetOrderRequest<S: ExchangeSpec> {
    pub id: <S as ExchangeSpec>::OrderId,
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
pub struct CancelOrderRequest<S: ExchangeSpec> {
    pub id: <S as ExchangeSpec>::OrderId,
    pub market_pair: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug, PartialEq)]
pub struct CancelAllOrdersRequest {
    pub market_pair: Option<String>,
}

#[derive(Serialize, Deserialize, Constructor, Debug)]
pub struct GetOrderHistoryRequest<S: ExchangeSpec> {
    pub market_pair: Option<String>,
    pub paginator: Option<Paginator<S>>,
}

impl<S: ExchangeSpec> Clone for GetOrderHistoryRequest<S> {
    fn clone(&self) -> Self {
        Self {
            market_pair: self.market_pair.clone(),
            paginator: self.paginator.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct OrderCanceled<S: ExchangeSpec> {
    pub id: <S as ExchangeSpec>::OrderId,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct Trade<S: ExchangeSpec> {
    pub id: <S as ExchangeSpec>::TradeId,
    pub order_id: <S as ExchangeSpec>::OrderId,
    pub market_pair: String,
    pub price: Decimal,
    pub qty: Decimal,
    pub fees: Option<Decimal>,
    pub side: Side,
    pub liquidity: Option<Liquidity>,
    pub created_at: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Liquidity {
    Maker,
    Taker,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TradeHistoryRequest<S: ExchangeSpec> {
    pub market_pair: Option<String>,
    pub order_id: Option<<S as ExchangeSpec>::OrderId>,
    pub paginator: Option<Paginator<S>>,
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
pub struct GetHistoricRatesRequest<S: ExchangeSpec> {
    pub market_pair: String,
    pub paginator: Option<Paginator<S>>,
    pub interval: Interval,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct GetHistoricTradesRequest<S: ExchangeSpec> {
    pub market_pair: String,
    pub paginator: Option<Paginator<S>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum Interval {
    OneMinute,
    ThreeMinutes,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    OneHour,
    TwoHours,
    FourHours,
    SixHours,
    EightHours,
    TwelveHours,
    OneDay,
    ThreeDays,
    OneWeek,
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

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Paginator<S: ExchangeSpec> {
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u64>,
    pub before: Option<<S as ExchangeSpec>::Pagination>,
    pub after: Option<<S as ExchangeSpec>::Pagination>,
}

impl<S: ExchangeSpec> Clone for Paginator<S> {
    fn clone(&self) -> Self {
        Self {
            start_time: self.start_time.clone(),
            end_time: self.end_time.clone(),
            limit: self.limit.clone(),
            before: self.before.clone(),
            after: self.after.clone(),
        }
    }
}
