use derive_more::Constructor;
use rust_decimal::prelude::Decimal;
use serde::Deserialize;
use serde::Serialize;
use super::OrderStatus;
use super::OrderType;
use super::Side;
use super::Trade;

/// This struct represents an order
#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct Order {
    pub id: String,
    pub market_pair: String,
    pub client_order_id: Option<String>,
    pub created_at: Option<u64>,
    pub order_type: OrderType,
    pub side: Side,
    pub status: OrderStatus,
    pub size: Decimal,
    pub price: Option<Decimal>,
    pub remaining: Option<Decimal>,
    pub trades: Vec<Trade>,
}