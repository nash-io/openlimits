use async_trait::async_trait;
use shared::Result;

use crate::model::{OrderBookRequest, OrderBookResponse};

struct OpenLimits<E: Exchange> {
    exchange: E,
}

impl<E: Exchange> OpenLimits<E> {
    pub fn new(exchange: E) -> Self {
        Self { exchange }
    }

    pub async fn order_book(
        &mut self,
        req: impl AsRef<OrderBookRequest>,
    ) -> Result<OrderBookResponse> {
        self.exchange.order_book(req.as_ref()).await
    }
}

#[async_trait]
pub trait Exchange {
    async fn order_book(&mut self, req: &OrderBookRequest) -> Result<OrderBookResponse>;
}
