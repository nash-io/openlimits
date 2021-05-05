use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;
use super::shared::string_to_decimal;

/// This struct represents ask and bid
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AskBid {
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub qty: Decimal,
}