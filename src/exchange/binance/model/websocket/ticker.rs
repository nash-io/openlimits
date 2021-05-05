use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;
use super::shared::string_to_decimal;

/// This struct represents a ticker
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "p", with = "string_to_decimal")]
    pub price_change: Decimal,
    #[serde(rename = "P", with = "string_to_decimal")]
    pub price_change_percent: Decimal,
    #[serde(rename = "w", with = "string_to_decimal")]
    pub average_price: Decimal,
    #[serde(rename = "x", with = "string_to_decimal")]
    pub prev_close: Decimal,
    #[serde(rename = "c", with = "string_to_decimal")]
    pub current_close: Decimal,
    #[serde(rename = "Q", with = "string_to_decimal")]
    pub current_close_qty: Decimal,
    #[serde(rename = "b", with = "string_to_decimal")]
    pub best_bid: Decimal,
    #[serde(rename = "B", with = "string_to_decimal")]
    pub best_bid_qty: Decimal,
    #[serde(rename = "a", with = "string_to_decimal")]
    pub best_ask: Decimal,
    #[serde(rename = "A", with = "string_to_decimal")]
    pub best_ask_qty: Decimal,
    #[serde(rename = "o", with = "string_to_decimal")]
    pub open: Decimal,
    #[serde(rename = "h", with = "string_to_decimal")]
    pub high: Decimal,
    #[serde(rename = "l", with = "string_to_decimal")]
    pub low: Decimal,
    #[serde(rename = "v", with = "string_to_decimal")]
    pub volume: Decimal,
    #[serde(rename = "q", with = "string_to_decimal")]
    pub quote_volume: Decimal,
    #[serde(rename = "O")]
    pub open_time: u64,
    #[serde(rename = "C")]
    pub close_time: u64,
    #[serde(rename = "F")]
    pub first_trade_id: u64,
    #[serde(rename = "L")]
    pub last_trade_id: u64,
    #[serde(rename = "n")]
    pub num_trades: u64,
}
