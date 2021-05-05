use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;
use super::shared::string_to_decimal;

/// This struct represents a mini ticker
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MiniTicker {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c", with = "string_to_decimal")]
    pub close: Decimal,
    #[serde(rename = "o", with = "string_to_decimal")]
    pub open: Decimal,
    #[serde(rename = "l", with = "string_to_decimal")]
    pub low: Decimal,
    #[serde(rename = "h", with = "string_to_decimal")]
    pub high: Decimal,
    #[serde(rename = "v", with = "string_to_decimal")]
    pub volume: Decimal,
    #[serde(rename = "q", with = "string_to_decimal")]
    pub quote_volume: Decimal,
}