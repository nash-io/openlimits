use async_trait::async_trait;
use derive_more::{Deref, DerefMut};
use shared::Result;

use crate::exchange::Exchange;
use crate::model::{
    Asks, Bids, CancelAllOrdersRequest, CancelOrderRequest, OpenLimitOrderRequest,
    OpenMarketOrderRequest, Order, OrderBookRequest, OrderBookResponse, OrderCanceled,
};
use chrono::DateTime;

#[derive(Deref, DerefMut)]
pub struct Coinbase(coinbase::Coinbase);

impl Coinbase {
    pub fn new(sandbox: bool) -> Self {
        Coinbase(coinbase::Coinbase::new(sandbox))
    }

    pub fn with_credential(
        api_key: &str,
        api_secret: &str,
        passphrase: &str,
        sandbox: bool,
    ) -> Self {
        Coinbase(coinbase::Coinbase::with_credential(
            api_key, api_secret, passphrase, sandbox,
        ))
    }
}

#[async_trait]
impl Exchange for Coinbase {
    type IdType = String;
    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse> {
        self.book::<coinbase::model::BookRecordL2>(&req.symbol)
            .await
            .map(Into::into)
    }
    async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self::IdType>> {
        coinbase::Coinbase::limit_buy(self, &req.symbol, req.size, req.price)
            .await
            .map(Into::into)
    }
    async fn limit_sell(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self::IdType>> {
        coinbase::Coinbase::limit_sell(self, &req.symbol, req.size, req.price)
            .await
            .map(Into::into)
    }

    async fn market_buy(&self, req: &OpenMarketOrderRequest) -> Result<Order<Self::IdType>> {
        coinbase::Coinbase::market_buy(self, &req.symbol, req.size)
            .await
            .map(Into::into)
    }
    async fn market_sell(&self, req: &OpenMarketOrderRequest) -> Result<Order<Self::IdType>> {
        coinbase::Coinbase::market_sell(self, &req.symbol, req.size)
            .await
            .map(Into::into)
    }

    async fn cancel_order(
        &self,
        req: &CancelOrderRequest<Self::IdType>,
    ) -> Result<OrderCanceled<Self::IdType>> {
        coinbase::Coinbase::cancel_order(self, req.id.clone(), req.pair.as_deref())
            .await
            .map(Into::into)
    }

    async fn cancel_all_orders(
        &self,
        req: &CancelAllOrdersRequest,
    ) -> Result<Vec<OrderCanceled<Self::IdType>>> {
        coinbase::Coinbase::cancel_all_orders(self, req.pair.as_deref())
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_all_open_orders(&self) -> Result<Vec<Order<Self::IdType>>> {
        coinbase::Coinbase::get_all_open_orders(self)
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }
}

impl From<coinbase::model::Book<coinbase::model::BookRecordL2>> for OrderBookResponse {
    fn from(book: coinbase::model::Book<coinbase::model::BookRecordL2>) -> Self {
        Self {
            last_update_id: None,
            bids: book.bids.into_iter().map(Into::into).collect(),
            asks: book.asks.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<coinbase::model::BookRecordL2> for Bids {
    fn from(bids: coinbase::model::BookRecordL2) -> Self {
        Self {
            price: bids.price,
            qty: bids.size,
        }
    }
}

impl From<coinbase::model::BookRecordL2> for Asks {
    fn from(bids: coinbase::model::BookRecordL2) -> Self {
        Self {
            price: bids.price,
            qty: bids.size,
        }
    }
}

impl From<coinbase::model::Order> for Order<String> {
    fn from(order: coinbase::model::Order) -> Self {
        Self {
            id: order.id,
            symbol: order.product_id,
            client_order_id: None,
            created_at: DateTime::parse_from_rfc3339(&order.created_at)
                .unwrap()
                .into(),
        }
    }
}

impl From<String> for OrderCanceled<String> {
    fn from(id: String) -> Self {
        Self { id }
    }
}
