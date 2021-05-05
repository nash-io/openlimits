use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;
use super::shared::string_to_decimal;

/// This struct represents the symbol price
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymbolPrice {
    pub symbol: String,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
}