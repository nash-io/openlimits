use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;
use super::OrderStopType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderStop {
    stop_price: Decimal,
    #[serde(rename = "stop")]
    _type: OrderStopType,
}