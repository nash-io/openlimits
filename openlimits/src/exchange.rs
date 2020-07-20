use async_trait::async_trait;
use shared::Result;

use crate::model::{OpenLimitOrderRequest, Order, OrderBookRequest, OrderBookResponse};

pub struct OpenLimits<E: Exchange> {
    exchange: E,
}

impl<E: Exchange> OpenLimits<E> {
    pub fn new(exchange: E) -> Self {
        Self { exchange }
    }

    pub async fn order_book(self, req: impl AsRef<OrderBookRequest>) -> Result<OrderBookResponse> {
        self.exchange.order_book(req.as_ref()).await
    }

    pub async fn limit_buy(
        self,
        req: impl AsRef<OpenLimitOrderRequest>,
    ) -> Result<Order<E::IdType>> {
        self.exchange.limit_buy(req.as_ref()).await
    }
}

#[async_trait]
pub trait Exchange {
    type IdType;
    async fn order_book(self, req: &OrderBookRequest) -> Result<OrderBookResponse>;
    async fn limit_buy(self, req: &OpenLimitOrderRequest) -> Result<Order<Self::IdType>>;
}
