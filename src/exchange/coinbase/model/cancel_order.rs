use serde::Deserialize;
use serde::Serialize;

/// This struct represents the cancellation of an order
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancelOrder {
    pub product_id: Option<String>,
}