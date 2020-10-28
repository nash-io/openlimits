use super::{OrderBookResponse, Trade};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Subscription {
    Ticker(String),    // symbol
    OrderBook(String), // symbol
    Trades(String),    // symbol
    UserOrders,
    UserTrades,
}

#[derive(Debug, Clone, Serialize)]
pub enum OpenLimitsWebsocketMessage {
    Ping,
    OrderBook(OrderBookResponse),
    Trades(Vec<Trade>),
}
