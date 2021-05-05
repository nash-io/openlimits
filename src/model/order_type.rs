use serde::Deserialize;
use serde::Serialize;

/// This enum represents an order type
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
    StopLimit,
    StopMarket,
    Unknown,
}