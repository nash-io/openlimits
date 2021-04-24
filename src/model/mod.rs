use chrono::Duration;
use derive_more::Constructor;
use rust_decimal::prelude::Decimal;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;

#[cfg(feature = "python")]
pub mod python;

pub mod websocket;

#[derive(Serialize, Deserialize, Clone, Constructor, Debug, Default, PartialEq)]
pub struct OrderBookRequest {
    pub market_pair: String,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug, Default, PartialEq)]
pub struct OrderBookResponse {
    pub update_id: Option<u64>,
    pub last_update_id: Option<u64>,
    pub bids: Vec<AskBid>,
    pub asks: Vec<AskBid>,
}

#[derive(Serialize, Deserialize, Copy, Clone, Constructor, Debug, Default, PartialEq)]
pub struct AskBid {
    pub price: Decimal,
    pub qty: Decimal,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TimeInForce {
    GoodTillCancelled,
    ImmediateOrCancelled,
    FillOrKill,
    // Representing 'good till time' as a duration works for both Nash and Coinbase
    GoodTillTime(Duration),
}

// chrono::Duration does not have a serde serialize/deserialize option
struct TimeInForceVisitor;

impl<'de> Visitor<'de> for TimeInForceVisitor {
    type Value = TimeInForce;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an string, either GTC, IOC, FOK, GTT,duration")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.starts_with("GTT,") {
            match v[4..].parse::<u64>() {
                Ok(v) => Ok(TimeInForce::GoodTillTime(Duration::milliseconds(v as i64))),
                _ => Err(E::custom(format!("Invalid GTG: {}", v))),
            }
        } else {
            match v {
                "GTC" => Ok(TimeInForce::GoodTillCancelled),
                "IOC" => Ok(TimeInForce::ImmediateOrCancelled),
                "FOK" => Ok(TimeInForce::FillOrKill),
                _ => Err(E::custom(format!("Invalid string: {}", v))),
            }
        }
    }
}

impl<'de> Deserialize<'de> for TimeInForce {
    fn deserialize<D>(deserializer: D) -> Result<TimeInForce, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TimeInForceVisitor)
    }
}

impl Serialize for TimeInForce {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match *self {
            TimeInForce::GoodTillCancelled => String::from("GTC"),
            TimeInForce::ImmediateOrCancelled => String::from("IOC"),
            TimeInForce::FillOrKill => String::from("FOK"),
            TimeInForce::GoodTillTime(d) => format!("GTT,{}", d.num_milliseconds()),
        };
        serializer.serialize_str(s.as_str())
    }
}

impl Default for TimeInForce {
    fn default() -> Self {
        TimeInForce::GoodTillCancelled
    }
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug, Default, PartialEq)]
pub struct OpenLimitOrderRequest {
    pub market_pair: String,
    pub size: Decimal,
    pub price: Decimal,
    pub time_in_force: TimeInForce,
    pub post_only: bool,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug, Default, PartialEq)]
pub struct OpenMarketOrderRequest {
    pub market_pair: String,
    pub size: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct Order {
    pub id: String,
    pub market_pair: String,
    pub client_order_id: Option<String>,
    pub created_at: Option<u64>,
    pub order_type: OrderType,
    pub side: Side,
    pub status: OrderStatus,
    pub size: Decimal,
    pub price: Option<Decimal>,
    pub remaining: Option<Decimal>,
    pub trades: Vec<Trade>,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct GetOrderRequest {
    pub id: String,
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
pub struct CancelOrderRequest {
    pub id: String,
    pub market_pair: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug, PartialEq)]
pub struct CancelAllOrdersRequest {
    pub market_pair: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct GetOrderHistoryRequest {
    pub market_pair: Option<String>,
    pub order_status: Option<Vec<OrderStatus>>,
    pub paginator: Option<Paginator>,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct OrderCanceled {
    pub id: String,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct Trade {
    pub id: String,
    pub buyer_order_id: Option<String>,
    pub seller_order_id: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountFees {
    pub maker: Decimal,
    pub taker: Decimal,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TradeHistoryRequest {
    pub market_pair: Option<String>,
    pub order_id: Option<String>,
    pub paginator: Option<Paginator>,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug, PartialEq)]
pub struct Balance {
    pub asset: String,
    pub total: Decimal,
    pub free: Decimal,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug, PartialEq)]
pub struct Ticker {
    pub price: Option<Decimal>,
    pub price_24h: Option<Decimal>,
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
pub struct GetHistoricRatesRequest {
    pub market_pair: String,
    pub paginator: Option<Paginator>,
    pub interval: Interval,
}

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct GetHistoricTradesRequest {
    pub market_pair: String,
    pub paginator: Option<Paginator>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
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
pub struct Paginator {
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u64>,
    pub before: Option<String>,
    pub after: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    StopLimit,
    StopMarket,
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::TimeInForce;
    use chrono::Duration;

    #[test]
    fn can_serialize_time_in_force() {
        let t = TimeInForce::GoodTillTime(Duration::hours(1));
        let serialized = serde_json::to_string(&t).expect("Couldn't serialize as JSON.");
        let deserialized: TimeInForce =
            serde_json::from_str(&serialized).expect("Couldn't deserialize JSON.");
        assert_eq!(t, deserialized);
    }
}
