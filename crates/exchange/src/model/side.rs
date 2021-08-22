use serde::Deserialize;
use serde::Serialize;

/// This struct represents buy-side and sell-side
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    Buy,
    Sell,
}

// FIXME: Should be changed to TryFrom.
impl From<String> for Side {
    fn from(side: String) -> Self {
        if side.to_lowercase() == "buy" {
            Side::Buy
        } else {
            Side::Sell
        }
    }
}
