use async_trait::async_trait;
use derive_more::Constructor;
use shared::Result;

use crate::model::{
    OpenLimitOrderRequest, OpenMarketOrderRequest, Order, OrderBookRequest, OrderBookResponse,
};

#[derive(Constructor)]
pub struct OpenLimits<E: Exchange> {
    exchange: E,
}

impl<E: Exchange> OpenLimits<E> {
    pub async fn order_book(&self, req: impl AsRef<OrderBookRequest>) -> Result<OrderBookResponse> {
        self.exchange.order_book(req.as_ref()).await
    }

    pub async fn limit_buy(
        &self,
        req: impl AsRef<OpenLimitOrderRequest>,
    ) -> Result<Order<E::IdType>> {
        self.exchange.limit_buy(req.as_ref()).await
    }

    pub async fn limit_sell(
        &self,
        req: impl AsRef<OpenLimitOrderRequest>,
    ) -> Result<Order<E::IdType>> {
        self.exchange.limit_sell(req.as_ref()).await
    }

    pub async fn market_buy(
        &self,
        req: impl AsRef<OpenMarketOrderRequest>,
    ) -> Result<Order<E::IdType>> {
        self.exchange.market_buy(req.as_ref()).await
    }

    pub async fn market_sell(
        &self,
        req: impl AsRef<OpenMarketOrderRequest>,
    ) -> Result<Order<E::IdType>> {
        self.exchange.market_sell(req.as_ref()).await
    }

    // pub async fn cancel_order(
    //     &self,
    //     req: impl AsRef<CancelOrderRequest>,
    // ) -> Result<Order<E::IdType>> {
    //     self.exchange.cancel_order(req.as_ref()).await
    // }
}

#[async_trait]
pub trait Exchange {
    type IdType;
    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse>;
    async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self::IdType>>;
    async fn limit_sell(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self::IdType>>;
    async fn market_buy(&self, req: &OpenMarketOrderRequest) -> Result<Order<Self::IdType>>;
    async fn market_sell(&self, req: &OpenMarketOrderRequest) -> Result<Order<Self::IdType>>;
}
