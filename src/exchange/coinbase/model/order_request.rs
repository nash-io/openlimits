use serde::Deserialize;
use serde::Serialize;
use super::OrderSide;
use super::OrderRequestType;
use super::OrderStop;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrderRequest {
    pub side: OrderSide,
    pub client_oid: Option<String>,
    pub product_id: String,
    #[serde(flatten)]
    pub _type: OrderRequestType,
    #[serde(flatten)]
    pub stop: Option<OrderStop>,
}