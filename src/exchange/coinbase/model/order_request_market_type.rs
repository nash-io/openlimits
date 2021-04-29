use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum OrderRequestMarketType {
    Size { size: Decimal },
    Funds { funds: Decimal },
}