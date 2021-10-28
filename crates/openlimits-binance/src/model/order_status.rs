use serde::Deserialize;
use serde::Serialize;

/// This enum represents an order status
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    PendingCancel,
    Rejected,
    Expired,
}

impl Default for OrderStatus {
    fn default() -> Self {
        Self::New
    }
}