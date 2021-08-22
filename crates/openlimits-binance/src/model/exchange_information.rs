use serde::Deserialize;
use serde::Serialize;
use super::ExchangeFilter;
use super::RateLimit;
use super::Symbol;

/// This struct represents informations about the openlimits-exchange
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformation {
    pub timezone: String,
    pub server_time: u64,
    pub rate_limits: Vec<RateLimit>,
    pub exchange_filters: Vec<ExchangeFilter>,
    pub symbols: Vec<Symbol>,
}