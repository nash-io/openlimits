use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;
use super::shared::string_to_decimal;
use super::OrderExecType;
use super::OrderRejectReason;
use super::OrderStatus;
use super::OrderType;
use super::Side;
use super::TimeInForce;

/// This struct represents the user order update
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserOrderUpdate {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "c")]
    pub new_client_order_id: String,
    #[serde(rename = "S")]
    pub side: Side,
    #[serde(rename = "o")]
    pub order_type: OrderType,
    #[serde(rename = "f")]
    pub time_in_force: TimeInForce,
    #[serde(rename = "q", with = "string_to_decimal")]
    pub qty: Decimal,
    #[serde(rename = "p", with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(rename = "P", with = "string_to_decimal")]
    pub stop_price: Decimal,
    #[serde(rename = "F", with = "string_to_decimal")]
    pub iceberg_qty: Decimal,
    #[serde(skip_serializing)]
    pub g: i32,
    #[serde(skip_serializing, rename = "C")]
    pub c_ignore: Option<String>,
    #[serde(rename = "x")]
    pub execution_type: OrderExecType,
    #[serde(rename = "X")]
    pub order_status: OrderStatus,
    #[serde(rename = "r")]
    pub order_reject_reason: OrderRejectReason,
    #[serde(rename = "i")]
    pub order_id: u64,
    #[serde(rename = "l", with = "string_to_decimal")]
    pub qty_last_filled_trade: Decimal,
    #[serde(rename = "z", with = "string_to_decimal")]
    pub accumulated_qty_filled_trades: Decimal,
    #[serde(rename = "L", with = "string_to_decimal")]
    pub price_last_filled_trade: Decimal,
    #[serde(rename = "n", with = "string_to_decimal")]
    pub commission: Decimal,
    #[serde(skip_serializing, rename = "N")]
    pub asset_commisioned: Option<String>,
    #[serde(rename = "T")]
    pub trade_order_time: u64,
    #[serde(rename = "t")]
    pub trade_id: i64,
    #[serde(skip_serializing, rename = "I")]
    pub i_ignore: u64,
    #[serde(skip_serializing)]
    pub w: bool,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(skip_serializing, rename = "M")]
    pub m_ignore: bool,
    #[serde(skip_serializing, rename = "O")]
    pub order_creation_time: u64,
    #[serde(skip_serializing, rename = "Z", with = "string_to_decimal")]
    pub cumulative_quote_asset_transacted_qty: Decimal,
}