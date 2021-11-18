use serde::Deserialize;
use serde::Serialize;
use chrono::naive::NaiveDateTime;
use rust_decimal::prelude::Decimal;
use super::OrderSide;
use super::OrderType;
use super::OrderStatus;
use super::OrderStop;
use super::shared::string_to_decimal;
use super::shared::naive_datetime_from_string;

/// This struct represents an order
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    pub id: String,
    pub product_id: String,
    pub side: OrderSide,
    pub stp: Option<String>,
    #[serde(flatten)]
    pub _type: OrderType,
    pub post_only: bool,
    #[serde(with = "naive_datetime_from_string")]
    pub created_at: NaiveDateTime,
    #[serde(with = "string_to_decimal")]
    pub fill_fees: Decimal,
    #[serde(with = "string_to_decimal")]
    pub filled_size: Decimal,
    #[serde(with = "string_to_decimal")]
    pub executed_value: Decimal,
    pub status: OrderStatus,
    pub settled: bool,
    #[serde(flatten)]
    pub stop: Option<OrderStop>,
}