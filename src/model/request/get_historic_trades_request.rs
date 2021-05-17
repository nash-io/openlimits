use derive_more::Constructor;
use serde::Deserialize;
use serde::Serialize;
use crate::model::Paginator;

/// This struct represents the historic of the trades
#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct GetHistoricTradesRequest {
    pub market_pair: String,
    pub paginator: Option<Paginator>,
}