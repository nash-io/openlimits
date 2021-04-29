use crate::shared::string_to_decimal;
use rust_decimal::prelude::Decimal;
use serde::Deserialize;
use super::OrderSide;

#[derive(Deserialize, Debug, Clone)]
pub struct Level2UpdateRecord {
    pub side: OrderSide,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub size: Decimal,
}