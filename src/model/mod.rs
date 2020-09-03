use derive_more::Constructor;
use rust_decimal::prelude::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Constructor, Debug, Default)]
pub struct OrderBookRequest {
    pub market_pair: String,
}

#[derive(Clone, Constructor, Debug, Default)]
pub struct OrderBookResponse {
    pub last_update_id: Option<u64>,
    pub bids: Vec<AskBid>,
    pub asks: Vec<AskBid>,
}

#[derive(Clone, Constructor, Debug, Default)]
pub struct AskBid {
    pub price: Decimal,
    pub qty: Decimal,
}

#[derive(Clone, Constructor, Debug, Default)]
pub struct OpenLimitOrderRequest {
    pub market_pair: String,
    pub size: Decimal,
    pub price: Decimal,
}

#[derive(Clone, Constructor, Debug, Default)]
pub struct OpenMarketOrderRequest {
    pub market_pair: String,
    pub size: Decimal,
}

#[derive(Clone, Constructor, Debug)]
pub struct Order<T> {
    pub id: T,
    pub market_pair: String,
    pub client_order_id: Option<String>,
    pub created_at: Option<u64>,
    pub order_type: String,
    pub side: Side,
    pub status: OrderStatus,
    pub size: Decimal,
    pub price: Option<Decimal>,
}

#[derive(Clone, Constructor, Debug)]
pub struct GetOrderRequest<T> {
    pub id: T,
    pub market_pair: Option<String>,
}

#[derive(Clone, Constructor, Debug)]
pub struct Transaction<T> {
    pub id: T,
    pub market_pair: String,
    pub client_order_id: Option<String>,
    pub created_at: u64,
}

#[derive(Clone, Constructor, Debug)]
pub struct CancelOrderRequest<T> {
    pub id: T,
    pub market_pair: Option<String>,
}

#[derive(Clone, Constructor, Debug)]
pub struct CancelAllOrdersRequest {
    pub market_pair: Option<String>,
}

#[derive(Clone, Constructor, Debug)]
pub struct GetOrderHistoryRequest<T> {
    pub market_pair: Option<String>,
    pub paginator: Option<Paginator<T>>,
}

#[derive(Clone, Constructor, Debug)]
pub struct OrderCanceled<T> {
    pub id: T,
}

#[derive(Clone, Constructor, Debug)]
pub struct Trade<T, O> {
    pub id: T,
    pub order_id: O,
    pub market_pair: String,
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
pub struct TradeHistoryRequest<T, U> {
    pub market_pair: Option<String>,
    pub order_id: Option<T>,
    pub paginator: Option<Paginator<U>>,
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

#[derive(Clone, Constructor, Debug)]
pub struct Candle {
    pub time: u64,
    pub low: Decimal,
    pub high: Decimal,
    pub open: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}

#[derive(Clone, Constructor, Debug, Default)]
pub struct GetPriceTickerRequest {
    pub market_pair: String,
}

#[derive(Clone, Constructor, Debug)]
pub struct GetHistoricRatesRequest<T> {
    pub market_pair: String,
    pub paginator: Option<Paginator<T>>,
    pub interval: Interval,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Interval {
    OneMinute,
    ThreeMinutes,
    FiveMinutes,
    FiftyMinutes,
    ThirtyMinutes,
    OneHour,
    TwoHours,
    FourHours,
    SixHours,
    EightHours,
    TwelveHours,
    OneDay,
    ThreeDay,
    OneWeek,
    OneMonth,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Paginator<T> {
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u64>,
    pub after: Option<T>,
    pub before: Option<T>,
}
