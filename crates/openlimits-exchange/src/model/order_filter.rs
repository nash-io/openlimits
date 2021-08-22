use rust_decimal::prelude::Decimal;
use serde::Deserialize;
use serde::Serialize;
use super::OrderStatus;
use super::OrderType;
use super::Side;
use std::ops::Range;

/// This struct represents an order
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
#[allow(missing_docs)]
pub struct OrderFilter {
    pub market_pair: Option<String>,
    pub client_order_id: Option<String>,
    pub order_type: Option<OrderType>,
    pub side: Option<Side>,
    pub status: Option<OrderStatus>,
    pub created_at: Option<Range<u64>>,
    pub size: Option<Range<Decimal>>,
    pub price: Option<Range<Decimal>>,
    pub remaining: Option<Range<Decimal>>,
}

impl OrderFilter {
    /// Creates an OrderFilter without any filtering rule.
    pub fn new() -> Self {
        Default::default()
    }

    /// Set market pair.
    pub fn with_market_pair(mut self, market_pair: Option<String>) -> Self {
        self.market_pair = market_pair;
        self
    }

    /// Set client order ID.
    pub fn with_client_order_id(mut self, client_order_id: Option<String>) -> Self {
        self.client_order_id = client_order_id;
        self
    }

    /// Set OrderType.
    pub fn with_order_type(mut self, order_type: Option<OrderType>) -> Self {
        self.order_type = order_type;
        self
    }

    /// Set Side.
    pub fn with_side(mut self, side: Option<Side>) -> Self {
        self.side = side;
        self
    }

    /// Set OrderStatus.
    pub fn with_status(mut self, status: Option<OrderStatus>) -> Self {
        self.status = status;
        self
    }

    /// Set creation time.
    pub fn with_created_at(mut self, created_at: Option<Range<u64>>) -> Self {
        self.created_at = created_at;
        self
    }

    /// Set size.
    pub fn with_size(mut self, size: Option<Range<Decimal>>) -> Self {
        self.size = size;
        self
    }

    /// Set price.
    pub fn with_price(mut self, price: Option<Range<Decimal>>) -> Self {
        self.price = price;
        self
    }

    /// Set remaining.
    pub fn with_remaining(mut self, remaining: Option<Range<Decimal>>) -> Self {
        self.remaining = remaining;
        self
    }
}
