use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;
use super::shared::string_to_decimal;

/// This struct represents a trade message
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeMessage {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "t")]
    pub trade_id: u64,
    #[serde(rename = "p", with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(rename = "q", with = "string_to_decimal")]
    pub qty: Decimal,
    #[serde(rename = "b")]
    pub buyer_order_id: u64,
    #[serde(rename = "a")]
    pub seller_order_id: u64,
    #[serde(rename = "T")]
    pub trade_order_time: u64,
    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
    #[serde(skip_serializing, rename = "M")]
    pub m_ignore: bool,
}