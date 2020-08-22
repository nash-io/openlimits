use async_trait::async_trait;
use derive_more::Constructor;

use crate::{
    shared::Result,
    openlimits::model::{
        Balance,
        CancelAllOrdersRequest,
        CancelOrderRequest,
        Candle,
        GetHistoricRatesRequest,
        GetOrderHistoryRequest,
        GetPriceTickerRequest,
        OpenLimitOrderRequest,
        OpenMarketOrderRequest,
        Order,
        OrderBookRequest,
        OrderBookResponse,
        OrderCanceled,
        Paginator,
        Ticker,
        Trade,
        TradeHistoryRequest,
    },
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
    ) -> Result<Order<E::OrderIdType>> {
        self.exchange.limit_buy(req.as_ref()).await
    }

    pub async fn limit_sell(
        &self,
        req: impl AsRef<OpenLimitOrderRequest>,
    ) -> Result<Order<E::OrderIdType>> {
        self.exchange.limit_sell(req.as_ref()).await
    }

    pub async fn market_buy(
        &self,
        req: impl AsRef<OpenMarketOrderRequest>,
    ) -> Result<Order<E::OrderIdType>> {
        self.exchange.market_buy(req.as_ref()).await
    }

    pub async fn market_sell(
        &self,
        req: impl AsRef<OpenMarketOrderRequest>,
    ) -> Result<Order<E::OrderIdType>> {
        self.exchange.market_sell(req.as_ref()).await
    }

    pub async fn cancel_order(
        &self,
        req: impl AsRef<CancelOrderRequest<E::OrderIdType>>,
    ) -> Result<OrderCanceled<E::OrderIdType>> {
        self.exchange.cancel_order(req.as_ref()).await
    }

    pub async fn cancel_all_orders(
        &self,
        req: impl AsRef<CancelAllOrdersRequest>,
    ) -> Result<Vec<OrderCanceled<E::OrderIdType>>> {
        self.exchange.cancel_all_orders(req.as_ref()).await
    }

    pub async fn get_all_open_orders(&self) -> Result<Vec<Order<E::OrderIdType>>> {
        self.exchange.get_all_open_orders().await
    }

    pub async fn get_order_history(
        &self,
        req: impl AsRef<GetOrderHistoryRequest>,
    ) -> Result<Vec<Order<E::OrderIdType>>> {
        self.exchange.get_order_history(req.as_ref()).await
    }

    pub async fn get_account_balances(
        &self,
        paginator: Option<&Paginator>,
    ) -> Result<Vec<Balance>> {
        self.exchange.get_account_balances(paginator).await
    }

    pub async fn get_trade_history(
        &self,
        req: impl AsRef<TradeHistoryRequest<E::OrderIdType>>,
    ) -> Result<Vec<Trade<E::TradeIdType, E::OrderIdType>>> {
        self.exchange.get_trade_history(req.as_ref()).await
    }

    pub async fn get_historic_rates(
        &self,
        req: impl AsRef<GetHistoricRatesRequest>,
    ) -> Result<Vec<Candle>> {
        self.exchange.get_historic_rates(req.as_ref()).await
    }
}

#[async_trait]
pub trait Exchange {
    type OrderIdType;
    type TradeIdType;
    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse>;
    async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self::OrderIdType>>;
    async fn limit_sell(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self::OrderIdType>>;
    async fn market_buy(&self, req: &OpenMarketOrderRequest) -> Result<Order<Self::OrderIdType>>;
    async fn market_sell(&self, req: &OpenMarketOrderRequest) -> Result<Order<Self::OrderIdType>>;
    async fn cancel_order(
        &self,
        req: &CancelOrderRequest<Self::OrderIdType>,
    ) -> Result<OrderCanceled<Self::OrderIdType>>;
    async fn cancel_all_orders(
        &self,
        req: &CancelAllOrdersRequest,
    ) -> Result<Vec<OrderCanceled<Self::OrderIdType>>>;
    async fn get_all_open_orders(&self) -> Result<Vec<Order<Self::OrderIdType>>>;
    async fn get_order_history(
        &self,
        req: &GetOrderHistoryRequest,
    ) -> Result<Vec<Order<Self::OrderIdType>>>;
    async fn get_account_balances(&self, paginator: Option<&Paginator>) -> Result<Vec<Balance>>;
    async fn get_trade_history(
        &self,
        req: &TradeHistoryRequest<Self::OrderIdType>,
    ) -> Result<Vec<Trade<Self::TradeIdType, Self::OrderIdType>>>;
    async fn get_price_ticker(&self, req: &GetPriceTickerRequest) -> Result<Ticker>;
    async fn get_historic_rates(&self, req: &GetHistoricRatesRequest) -> Result<Vec<Candle>>;
}
