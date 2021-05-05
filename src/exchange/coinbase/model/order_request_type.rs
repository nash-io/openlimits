use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;
use super::OrderTimeInForce;
use super::OrderRequestMarketType;

/// This struct represents a order type request
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum OrderRequestType {
    Limit {
        price: Decimal,
        size: Decimal,
        post_only: bool,
        #[serde(flatten)]
        time_in_force: Option<OrderTimeInForce>,
    },
    Market {
        #[serde(flatten)]
        _type: OrderRequestMarketType,
    },
}