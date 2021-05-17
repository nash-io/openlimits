use rust_decimal::prelude::Decimal;
use serde::Deserialize;
use super::OrderSide;
use super::shared::string_to_decimal;
use super::shared::string_to_opt_decimal;

#[derive(Deserialize, Debug, Clone)]
pub struct Change {
    pub time: String,
    pub sequence: usize,
    pub order_id: String,
    pub product_id: String,
    #[serde(with = "string_to_decimal")]
    pub new_size: Decimal,
    #[serde(with = "string_to_decimal")]
    pub old_size: Decimal,
    #[serde(default)]
    #[serde(with = "string_to_opt_decimal")]
    pub new_funds: Option<Decimal>,
    #[serde(default)]
    #[serde(with = "string_to_opt_decimal")]
    pub old_funds: Option<Decimal>,
    #[serde(default)]
    #[serde(with = "string_to_opt_decimal")]
    pub price: Option<Decimal>,
    pub side: OrderSide,
    pub user_id: Option<String>,
    #[serde(default)]
    pub profile_id: Option<String>,
}