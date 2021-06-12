use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;
use super::shared::string_to_decimal;

/// This struct represents price statistics 
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceStats {
    pub symbol: String,
    #[serde(with = "string_to_decimal")]
    pub price_change: Decimal,
    #[serde(with = "string_to_decimal")]
    pub price_change_percent: Decimal,
    #[serde(with = "string_to_decimal")]
    pub weighted_avg_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub prev_close_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub last_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub bid_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub ask_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub open_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub high_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub low_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub volume: Decimal,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: i64, // For dummy symbol "123456", it is -1
    pub last_id: i64,  // Same as above
    pub count: u64,
}