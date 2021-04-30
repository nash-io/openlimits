use rust_decimal::prelude::Decimal;
use serde::Deserialize;
use super::shared::string_to_decimal;

#[derive(Deserialize, Debug, Clone)]
pub struct Level2SnapshotRecord {
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub size: Decimal,
}