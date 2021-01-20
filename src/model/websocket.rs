use super::{OrderBookResponse, Trade};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Subscription {
    Ticker(String),           // symbol
    OrderBookUpdates(String), // symbol
    Trades(String),           // symbol
    AccountOrders(String),    // symbol
    AccountTrades(String),    // symbol
    AccountBalance(String),   // symbol
}

#[derive(Debug, Clone)]
pub enum WebSocketResponse<T> {
    Generic(OpenLimitsWebSocketMessage),
    Raw(T),
}

#[derive(Debug, Clone, Serialize)]
pub enum OpenLimitsWebSocketMessage {
    Ping,
    OrderBook(OrderBookResponse),
    OrderBookDiff(OrderBookResponse),
    Trades(Vec<Trade>),
}
