use derive_more::Constructor;
use serde::Deserialize;
use serde::Serialize;

/// This struct represents the ticker price.
#[derive(Serialize, Deserialize, Clone, Constructor, Debug, Default, PartialEq)]
pub struct GetPriceTickerRequest {
    pub market_pair: String,
}