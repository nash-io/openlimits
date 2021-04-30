use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;
use super::OrderStatus;
use super::shared::string_to_decimal;
use super::shared::string_to_opt_decimal;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub symbol: String,
    pub order_id: u64,
    pub client_order_id: String,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub orig_qty: Decimal,
    #[serde(with = "string_to_decimal")]
    pub executed_qty: Decimal,
    pub status: OrderStatus,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub side: String,
    #[serde(with = "string_to_opt_decimal")]
    #[serde(default)]
    pub stop_price: Option<Decimal>,
    #[serde(default)]
    pub iceberg_qty: Option<String>,
    #[serde(default)]
    pub time: Option<u64>,
}