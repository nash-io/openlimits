use super::{OrderBookResponse, Trade};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Subscription {
    UserData(String),            // listen key
    AggregateTrade(String),      // symbol
    Trade(String),               // symbol
    Candlestick(String, String), // (symbol, interval)
    MiniTicker(String),          // symbol
    MiniTickerAll,
    Ticker(String), // symbol
    TickerAll,
    OrderBook(String, i64),     // (symbol, depth)
    Depth(String, Option<u16>), // (symbol, interval)
}

#[derive(Debug, Clone, Serialize)]
pub enum OpenLimitsWebsocketMessage {
    Ping,
    OrderBook(OrderBookResponse),
    Trades(Vec<Trade<String, String>>),
}
