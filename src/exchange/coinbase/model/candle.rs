use serde::Deserialize;
use serde::Serialize;
use rust_decimal::prelude::Decimal;

/// This struct represents the candle
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Candle {
    pub time: u64,
    pub low: Decimal,
    pub high: Decimal,
    pub open: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
}