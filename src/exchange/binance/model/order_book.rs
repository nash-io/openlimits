use serde::Deserialize;
use serde::Serialize;
use super::AskBid;

/// This struct represents a order book
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,
    pub bids: Vec<AskBid>,
    pub asks: Vec<AskBid>,
}