use crate::shared::string_to_decimal;
use rust_decimal::prelude::Decimal;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Level2SnapshotRecord {
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub size: Decimal,
}