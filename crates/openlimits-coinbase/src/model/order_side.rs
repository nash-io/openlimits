use serde::Deserialize;
use serde::Serialize;

/// This enum represents an order side which can be buy-side or sell-side
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum OrderSide {
    Buy,
    Sell,
}