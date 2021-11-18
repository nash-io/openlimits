use rust_decimal::prelude::Decimal;
use serde::Deserialize;
use super::StopType;
use super::shared::string_to_decimal;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Activate {
    pub product_id: String,
    #[serde(with = "string_to_decimal")]
    pub timestamp: Decimal,
    pub order_id: String,
    pub stop_type: StopType,
    #[serde(with = "string_to_decimal")]
    pub size: Decimal,
    #[serde(with = "string_to_decimal")]
    pub funds: Decimal,
    #[serde(with = "string_to_decimal")]
    pub taker_fee_rate: Decimal,
    pub private: bool,
    pub user_id: Option<String>,
    #[serde(default)]
    pub profile_id: Option<String>,
}