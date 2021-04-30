use rust_decimal::prelude::Decimal;
use serde::Deserialize;
use super::OrderSide;
use super::shared::string_to_decimal;

#[derive(Deserialize, Debug, Clone)]
pub struct Match {
    pub trade_id: usize,
    pub sequence: usize,
    pub maker_order_id: String,
    pub taker_order_id: String,
    pub time: String,
    pub product_id: String,
    #[serde(with = "string_to_decimal")]
    pub size: Decimal,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    pub side: OrderSide,
    pub taker_user_id: Option<String>,
    pub taker_profile_id: Option<String>,
    pub maker_user_id: Option<String>,
    pub maker_profile_id: Option<String>,
    pub user_id: Option<String>,
    #[serde(default)]
    pub profile_id: Option<String>,
}