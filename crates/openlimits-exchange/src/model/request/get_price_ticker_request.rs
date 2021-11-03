use derive_more::Constructor;
use serde::Deserialize;
use serde::Serialize;
use crate::model::market_pair::MarketPair;

/// This struct represents the ticker price.
#[derive(Serialize, Deserialize, Clone, Constructor, Debug, PartialEq)]
pub struct GetPriceTickerRequest {
    pub market_pair: MarketPair,
}