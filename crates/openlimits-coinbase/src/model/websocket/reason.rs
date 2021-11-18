use serde::Deserialize;

/// This enum represents the reason why an order was rejected
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Reason {
    Filled,
    Canceled,
}