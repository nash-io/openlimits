use derive_more::Constructor;
use rust_decimal::prelude::Decimal;
use serde::Deserialize;
use serde::Serialize;
use crate::model::market_pair::MarketPair;

/// This struct represents an open market order
#[derive(Serialize, Deserialize, Clone, Constructor, Debug, PartialEq)]
pub struct OpenMarketOrderRequest {
    pub client_order_id: Option<String>,
    pub market_pair: MarketPair,
    pub size: Decimal,
}