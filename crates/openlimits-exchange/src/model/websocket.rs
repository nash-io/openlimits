use super::{OrderBookResponse, Trade};
use crate::model::{OrderStatus, OrderType, Side};
use serde::{Deserialize, Serialize};
use std::ops::Range;
use crate::model::market_pair::MarketPair;

/// This struct represents the account order
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountOrders {
    pub market: Option<MarketPair>,
    pub order_type: Option<Vec<OrderType>>,
    pub buy_or_sell: Option<Side>,
    pub range: Option<Range<u64>>,
    pub status: Option<Vec<OrderStatus>>,
}

/// This enum represents a subscription
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Subscription {
    // Ticker(MarketPair),           // symbol
    OrderBookUpdates(MarketPair), // symbol
    Trades(MarketPair),           // symbol
    // AccountTrades(MarketPair),    // symbol
    // AccountBalance(MarketPair),   // symbol
    // AccountOrders(AccountOrders),
}

/// This enum represents a websocket response
#[derive(Debug, Clone)]
pub enum WebSocketResponse<T> {
    Generic(OpenLimitsWebSocketMessage),
    Raw(T),
}

/// This enum represents a websocket message type
#[derive(Debug, Clone, Serialize)]
pub enum OpenLimitsWebSocketMessage {
    Ping,
    OrderBook(OrderBookResponse),
    // OrderBookDiff(OrderBookResponse),
    Trades(Vec<Trade>),
}
