use crate::shared::string_to_decimal;
use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;
use super::BookLevel;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BookRecordL1 {
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub size: Decimal,
    pub num_orders: usize,
}

impl BookLevel for BookRecordL1 {
    fn level() -> u8 {
        1
    }
}