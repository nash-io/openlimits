use std::ops::Deref;

use async_trait::async_trait;

use crate::{
    exchange_info::MarketPairHandle,
    model::{
        Balance, CancelAllOrdersRequest, CancelOrderRequest, Candle, GetHistoricRatesRequest,
        GetHistoricTradesRequest, GetOrderHistoryRequest, GetOrderRequest, GetPriceTickerRequest,
        OpenLimitOrderRequest, OpenMarketOrderRequest, Order, OrderBookRequest, OrderBookResponse,
        OrderCanceled, Paginator, Ticker, Trade, TradeHistoryRequest,
    },
    shared::Result,
};

pub struct OpenLimits {}

impl OpenLimits {
    pub async fn instantiate<Exc: Exchange + ExchangeInstantiation>(
        parameters: Exc::Parameters,
    ) -> ExchangeWrapper<Exc> {
        ExchangeWrapper::new(Exc::new(parameters).await)
    }
}

pub struct ExchangeWrapper<Exc: Exchange + ?Sized> {
    inner: Exc,
}

impl<Exc: Exchange> ExchangeWrapper<Exc> {
    pub fn new(inner: Exc) -> Self {
        Self { inner }
    }
}

impl<Exc: 'static + Exchange> Deref for ExchangeWrapper<Exc> {
    type Target =
        dyn Exchange<OrderId = Exc::OrderId, TradeId = Exc::TradeId, Pagination = Exc::Pagination>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[async_trait]
pub trait ExchangeInstantiation {
    type Parameters;

    async fn new(parameters: Self::Parameters) -> Self;
}

#[async_trait]
pub trait Exchange {
    type OrderId;
    type TradeId;
    type Pagination;

    async fn refresh_market_info(&self) -> Result<Vec<MarketPairHandle>>;
    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse>;
    async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self::OrderId>>;
    async fn limit_sell(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self::OrderId>>;
    async fn market_buy(&self, req: &OpenMarketOrderRequest) -> Result<Order<Self::OrderId>>;
    async fn market_sell(&self, req: &OpenMarketOrderRequest) -> Result<Order<Self::OrderId>>;
    async fn cancel_order(
        &self,
        req: &CancelOrderRequest<Self::OrderId>,
    ) -> Result<OrderCanceled<Self::OrderId>>;
    async fn cancel_all_orders(
        &self,
        req: &CancelAllOrdersRequest,
    ) -> Result<Vec<OrderCanceled<Self::OrderId>>>;
    async fn get_all_open_orders(&self) -> Result<Vec<Order<Self::OrderId>>>;
    async fn get_order_history(
        &self,
        req: &GetOrderHistoryRequest<Self::Pagination>,
    ) -> Result<Vec<Order<Self::OrderId>>>;
    async fn get_account_balances(
        &self,
        paginator: Option<&Paginator<Self::Pagination>>,
    ) -> Result<Vec<Balance>>;
    async fn get_trade_history(
        &self,
        req: &TradeHistoryRequest<Self::OrderId, Self::Pagination>,
    ) -> Result<Vec<Trade<Self::TradeId, Self::OrderId>>>;
    async fn get_price_ticker(&self, req: &GetPriceTickerRequest) -> Result<Ticker>;
    async fn get_historic_rates(
        &self,
        req: &GetHistoricRatesRequest<Self::Pagination>,
    ) -> Result<Vec<Candle>>;
    async fn get_historic_trades(
        &self,
        req: &GetHistoricTradesRequest<Self::Pagination>,
    ) -> Result<Vec<Trade<Self::TradeId, Self::OrderId>>>;
    async fn get_order(&self, req: &GetOrderRequest<Self::OrderId>)
        -> Result<Order<Self::OrderId>>;
}
