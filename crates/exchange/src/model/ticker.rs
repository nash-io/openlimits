use derive_more::Constructor;
use rust_decimal::prelude::Decimal;
use serde::Deserialize;
use serde::Serialize;

/// This struct represents a ticker
#[derive(Serialize, Deserialize, Clone, Constructor, Debug, PartialEq)]
pub struct Ticker {
    pub price: Option<Decimal>,
    pub price_24h: Option<Decimal>,
}