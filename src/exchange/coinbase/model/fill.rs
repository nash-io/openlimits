use crate::shared::naive_datetime_from_string;
use crate::shared::string_to_decimal;
use serde::Deserialize;
use serde::Serialize;
use chrono::naive::NaiveDateTime;
use rust_decimal::prelude::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Fill {
    pub trade_id: u64,
    pub product_id: String,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub size: Decimal,
    pub order_id: String,
    #[serde(with = "naive_datetime_from_string")]
    pub created_at: NaiveDateTime,
    pub liquidity: String,
    #[serde(with = "string_to_decimal")]
    pub fee: Decimal,
    pub settled: bool,
    pub side: String,
}