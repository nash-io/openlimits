use serde::Deserialize;
use serde::Serialize;

/// This struct represents the cancellation of all orders
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancelAllOrders {
    pub product_id: Option<String>,
}