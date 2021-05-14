use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;

/// This struct represents a market type request
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum OrderRequestMarketType {
    Size { size: Decimal },
    Funds { funds: Decimal },
}