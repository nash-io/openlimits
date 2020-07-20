use async_trait::async_trait;
use derive_more::{Deref, DerefMut};
use shared::Result;

use crate::exchange::Exchange;
use crate::model::{Asks, Bids, OpenLimitOrderRequest, Order, OrderBookRequest, OrderBookResponse};
use chrono::naive::NaiveDateTime;
use chrono::{DateTime, Utc};

#[derive(Deref, DerefMut)]
pub struct Binance(binance::Binance);

impl Binance {
    pub fn new(sandbox: bool) -> Self {
        Binance(binance::Binance::new(sandbox))
    }

    pub fn with_credential(api_key: &str, api_secret: &str, sandbox: bool) -> Self {
        Binance(binance::Binance::with_credential(
            api_key, api_secret, sandbox,
        ))
    }
}

#[async_trait]
impl Exchange for Binance {
    type IdType = u64;

    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse> {
        self.get_depth(req.symbol.as_str(), None)
            .await
            .map(Into::into)
    }

    async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self::IdType>> {
        binance::Binance::limit_buy(self, &req.symbol, req.size, req.price)
            .await
            .map(Into::into)
    }
}

impl From<binance::model::OrderBook> for OrderBookResponse {
    fn from(book: binance::model::OrderBook) -> Self {
        Self {
            last_update_id: Some(book.last_update_id),
            bids: book.bids.into_iter().map(Into::into).collect(),
            asks: book.asks.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<binance::model::Bids> for Bids {
    fn from(bids: binance::model::Bids) -> Self {
        Self {
            price: bids.price,
            qty: bids.qty,
        }
    }
}

impl From<binance::model::Asks> for Asks {
    fn from(bids: binance::model::Asks) -> Self {
        Self {
            price: bids.price,
            qty: bids.qty,
        }
    }
}

impl From<binance::model::Transaction> for Order<u64> {
    fn from(order: binance::model::Transaction) -> Self {
        let created_at = NaiveDateTime::from_timestamp(
            (order.transact_time / 1000) as i64,
            ((order.transact_time % 1000) * 1_000_000) as u32,
        );
        Self {
            id: order.order_id,
            symbol: order.symbol,
            client_order_id: Some(order.client_order_id),
            created_at: DateTime::from_utc(created_at, Utc),
        }
    }
}
