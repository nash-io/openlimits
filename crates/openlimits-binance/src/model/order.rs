use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;
use super::OrderStatus;
use super::shared::string_to_decimal;
use super::shared::string_to_opt_decimal;

/// This struct represents an order
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub symbol: String,
    pub order_id: u64,
    pub client_order_id: String,
    #[serde(default, with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(default, with = "string_to_decimal")]
    pub orig_qty: Decimal,
    #[serde(default, with = "string_to_decimal")]
    pub executed_qty: Decimal,
    #[serde(default)]
    pub status: OrderStatus,
    #[serde(default)]
    pub time_in_force: Option<String>,
    #[serde(default, rename = "type")]
    pub type_name: String,
    #[serde(default)]
    pub side: String,
    #[serde(default, with = "string_to_opt_decimal")]
    pub stop_price: Option<Decimal>,
    #[serde(default)]
    pub iceberg_qty: Option<String>,
    #[serde(default)]
    pub time: Option<u64>,
}