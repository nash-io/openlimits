use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;
use super::shared::string_to_decimal;

/// This struct represent the balance
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub asset: String,
    #[serde(with = "string_to_decimal")]
    pub free: Decimal,
    #[serde(with = "string_to_decimal")]
    pub locked: Decimal,
}