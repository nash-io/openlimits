use crate::shared::string_to_decimal;
use serde::Deserialize;
use serde::Serialize;

use rust_decimal::prelude::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    pub symbol: String,
    #[serde(with = "string_to_decimal")]
    pub bid_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub bid_qty: Decimal,
    #[serde(with = "string_to_decimal")]
    pub ask_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub ask_qty: Decimal,
}