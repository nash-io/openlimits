use serde::Deserialize;
use serde::Serialize;
use crate::model::Paginator;
use crate::model::market_pair::MarketPair;

/// This struct represents the trade history
#[derive(Serialize, Deserialize, Default)]
pub struct TradeHistoryRequest {
    pub market_pair: Option<MarketPair>,
    pub order_id: Option<String>,
    pub paginator: Option<Paginator>,
}