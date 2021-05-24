use derive_more::Constructor;
use serde::Deserialize;
use serde::Serialize;
use crate::model::AskBid;

/// This struct represents an order book response
#[derive(Serialize, Deserialize, Clone, Constructor, Debug, Default, PartialEq)]
pub struct OrderBookResponse {
    pub update_id: Option<u64>,
    pub last_update_id: Option<u64>,
    pub bids: Vec<AskBid>,
    pub asks: Vec<AskBid>,
}