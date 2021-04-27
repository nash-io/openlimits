use derive_more::Constructor;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Constructor, Debug, Default, PartialEq)]
pub struct OrderBookRequest {
    pub market_pair: String,
}