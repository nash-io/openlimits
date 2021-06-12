use serde::Deserialize;
use serde::Serialize;

/// This struct represent the buy-side and sell-side
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    Buy,
    Sell,
}