use super::{OrderBookResponse, Trade};
use crate::model::{OrderStatus, OrderType, Side, Result};
use serde::{Deserialize, Serialize};
use std::ops::Range;
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountOrders {
    pub market: Option<String>,
    pub order_type: Option<Vec<OrderType>>,
    pub buy_or_sell: Option<Side>,
    pub range: Option<Range<u64>>,
    pub status: Option<Vec<OrderStatus>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Subscription {
    Ticker(String),           // symbol
    OrderBookUpdates(String), // symbol
    Trades(String),           // symbol
    AccountTrades(String),    // symbol
    AccountBalance(String),   // symbol
    AccountOrders(AccountOrders),
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

impl TryFrom<OpenLimitsWebSocketMessage> for WebSocketResponse<OpenLimitsWebSocketMessage> {
    type Error = anyhow::Error;

    fn try_from(value: OpenLimitsWebSocketMessage) -> Result<Self> {
        Ok(WebSocketResponse::Generic(value))
    }
}
