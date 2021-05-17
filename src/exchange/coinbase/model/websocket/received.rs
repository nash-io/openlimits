use super::shared::string_to_decimal;
use super::shared::string_to_opt_decimal;
use rust_decimal::prelude::Decimal;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "order_type")]
#[serde(rename_all = "camelCase")]
pub enum Received {
    Limit {
        time: String,
        product_id: String,
        sequence: usize,
        order_id: String,
        client_oid: Option<String>,
        #[serde(with = "string_to_decimal")]
        size: Decimal,
        #[serde(with = "string_to_decimal")]
        price: Decimal,
        side: super::OrderSide,
        user_id: Option<String>,
        #[serde(default)]
        profile_id: Option<String>,
    },
    Market {
        time: String,
        product_id: String,
        sequence: usize,
        order_id: String,
        client_oid: Option<String>,
        #[serde(default)]
        #[serde(with = "string_to_opt_decimal")]
        funds: Option<Decimal>,
        side: super::OrderSide,
    },
}