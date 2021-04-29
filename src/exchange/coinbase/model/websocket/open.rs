use crate::shared::string_to_decimal;
use rust_decimal::prelude::Decimal;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Open {
    pub time: String,
    pub product_id: String,
    pub sequence: usize,
    pub order_id: String,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub remaining_size: Decimal,
    pub side: super::OrderSide,
    pub user_id: Option<String>,
    #[serde(default)]
    pub profile_id: Option<String>,
}