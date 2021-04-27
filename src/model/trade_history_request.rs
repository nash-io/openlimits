use serde::Deserialize;
use serde::Serialize;
use super::Paginator;

#[derive(Serialize, Deserialize, Default)]
pub struct TradeHistoryRequest {
    pub market_pair: Option<String>,
    pub order_id: Option<String>,
    pub paginator: Option<Paginator>,
}