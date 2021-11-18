use rust_decimal::prelude::Decimal;
use serde::Deserialize;
use super::OrderSide;
use super::shared::string_to_decimal;

/// This struct represents the level 2 update record
#[derive(Deserialize, Debug, Clone)]
pub struct Level2UpdateRecord {
    pub side: OrderSide,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub size: Decimal,
}