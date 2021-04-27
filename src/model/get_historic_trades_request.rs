use derive_more::Constructor;
use serde::Deserialize;
use serde::Serialize;
use super::Paginator;

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct GetHistoricTradesRequest {
    pub market_pair: String,
    pub paginator: Option<Paginator>,
}