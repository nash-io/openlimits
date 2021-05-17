use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;
use super::shared::string_to_decimal;

/// This struct represents an account
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: String,
    pub currency: String,
    #[serde(with = "string_to_decimal")]
    pub balance: Decimal,
    #[serde(with = "string_to_decimal")]
    pub available: Decimal,
    #[serde(with = "string_to_decimal")]
    pub hold: Decimal,
    pub profile_id: String,
}