use crate::shared::string_to_decimal;
use crate::shared::string_to_opt_decimal;
use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;
use super::TimeInForce;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    pub symbol: String,
    #[serde(with = "string_to_decimal")]
    pub quantity: Decimal,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "string_to_opt_decimal")]
    pub price: Option<Decimal>,
    #[serde(rename = "side")]
    pub order_side: String,
    #[serde(rename = "type")]
    pub order_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
}