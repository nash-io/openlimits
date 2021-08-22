use serde::Deserialize;
use serde::Serialize;

/// This enum represents the reason why an order was rejected
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderRejectReason {
    None,
}