use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;
use super::shared::string_to_decimal;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "filterType", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolFilter {
    #[serde(rename_all = "camelCase")]
    LotSize {
        #[serde(with = "string_to_decimal")]
        min_qty: Decimal,
        #[serde(with = "string_to_decimal")]
        max_qty: Decimal,
        #[serde(with = "string_to_decimal")]
        step_size: Decimal,
    },
    #[serde(rename_all = "camelCase")]
    PriceFilter {
        #[serde(with = "string_to_decimal")]
        min_price: Decimal,
        #[serde(with = "string_to_decimal")]
        max_price: Decimal,
        #[serde(with = "string_to_decimal")]
        tick_size: Decimal,
    },
    #[serde(rename_all = "camelCase")]
    MinNotional {
        #[serde(with = "string_to_decimal")]
        min_notional: Decimal,
    },
    #[serde(rename_all = "camelCase")]
    MaxNumAlgoOrders { max_num_algo_orders: u64 },
    #[serde(rename_all = "camelCase")]
    MaxPosition {
        #[serde(with = "string_to_decimal")]
        max_position: Decimal,
    },
    #[serde(rename_all = "camelCase")]
    MaxNumOrders { max_num_orders: u64 },
    #[serde(rename_all = "camelCase")]
    IcebergParts { limit: u64 },
    #[serde(rename_all = "camelCase")]
    PercentPrice {
        #[serde(with = "string_to_decimal")]
        multiplier_up: Decimal,
        #[serde(with = "string_to_decimal")]
        multiplier_down: Decimal,
        avg_price_mins: u64,
    },
    #[serde(rename_all = "camelCase")]
    MarketLotSize {
        #[serde(with = "string_to_decimal")]
        min_qty: Decimal,
        #[serde(with = "string_to_decimal")]
        max_qty: Decimal,
        #[serde(with = "string_to_decimal")]
        step_size: Decimal,
    },
}