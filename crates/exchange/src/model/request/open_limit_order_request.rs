use derive_more::Constructor;
use rust_decimal::prelude::Decimal;
use serde::Deserialize;
use serde::Serialize;
use crate::model::TimeInForce;

/// This struct represents an open limit order
#[derive(Serialize, Deserialize, Clone, Constructor, Debug, Default, PartialEq)]
pub struct OpenLimitOrderRequest {
    pub client_order_id: Option<String>,
    pub market_pair: String,
    pub size: Decimal,
    pub price: Decimal,
    pub time_in_force: TimeInForce,
    pub post_only: bool,
}