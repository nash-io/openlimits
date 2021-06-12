use serde::Deserialize;
use serde::Serialize;

/// This struct represents a cancel order
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderCanceled {
    pub symbol: String,
    pub orig_client_order_id: String,
    pub order_id: u64,
    pub client_order_id: String,
}