use derive_more::Constructor;
use rust_decimal::prelude::Decimal;
use serde::Deserialize;
use serde::Serialize;

/// This struct represents an open market order
#[derive(Serialize, Deserialize, Clone, Constructor, Debug, Default, PartialEq)]
pub struct OpenMarketOrderRequest {
    pub market_pair: String,
    pub size: Decimal,
}