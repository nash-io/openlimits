use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;
use super::shared::string_to_decimal;

/// This struct represents the account update balance
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdateBalance {
    #[serde(rename = "a")]
    pub asset: String,
    #[serde(rename = "f", with = "string_to_decimal")]
    pub free: Decimal,
    #[serde(rename = "l", with = "string_to_decimal")]
    pub locked: Decimal,
}