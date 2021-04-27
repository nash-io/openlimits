use derive_more::Constructor;
use serde::Deserialize;
use serde::Serialize;
use super::Paginator;
use super::Interval;


#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct GetHistoricRatesRequest {
    pub market_pair: String,
    pub paginator: Option<Paginator>,
    pub interval: Interval,
}