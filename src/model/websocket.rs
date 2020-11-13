use super::{OrderBookResponse, Trade};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Subscription {
    Ticker(String),    // symbol
    OrderBook(String), // symbol
    Trades(String),    // symbol
    UserOrders,
    UserTrades,
}

#[derive(Debug, Clone, Serialize)]
pub enum OpenLimitsWebSocketMessage {
    Ping,
    OrderBook(OrderBookResponse),
    Trades(Vec<Trade>),
}
