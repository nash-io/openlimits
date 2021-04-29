use crate::shared::string_to_decimal;
use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;
use super::BookLevel;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BookRecordL3 {
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub size: Decimal,
    pub order_id: String,
}

impl BookLevel for BookRecordL3 {
    fn level() -> u8 {
        3
    }
}