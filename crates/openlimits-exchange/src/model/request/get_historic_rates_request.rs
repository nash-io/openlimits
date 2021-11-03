use derive_more::Constructor;
use serde::Deserialize;
use serde::Serialize;
use crate::model::{Paginator, Interval};
use crate::model::market_pair::MarketPair;

/// This struct represents the historic of the rates
#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct GetHistoricRatesRequest {
    pub market_pair: MarketPair,
    pub paginator: Option<Paginator>,
    pub interval: Interval,
}