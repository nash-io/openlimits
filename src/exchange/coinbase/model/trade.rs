use crate::shared::naive_datetime_from_string;
use crate::shared::string_to_decimal;
use serde::Deserialize;
use serde::Serialize;
use chrono::naive::NaiveDateTime;
use rust_decimal::prelude::Decimal;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trade {
    pub trade_id: u64,
    #[serde(with = "naive_datetime_from_string")]
    pub time: NaiveDateTime,
    pub size: String,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    pub side: String,
}