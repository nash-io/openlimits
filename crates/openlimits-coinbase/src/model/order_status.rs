use serde::Deserialize;
use serde::Serialize;

/// This enum represents an order status
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum OrderStatus {
    Open,
    Done,
    Pending,
    Active,
    Rejected,
}