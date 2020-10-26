use std::fmt::Debug;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    exchange_info::{ExchangeInfoRetrieval, MarketPairHandle},
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
    pub async fn instantiate<Exc: ExchangeEssentials>(
        parameters: Exc::Parameters,
    ) -> Exchange<Exc> {
        Exchange::new(parameters).await
    }
}

pub enum ExchangeId {
    Binance,
    Coinbase,
    Nash,
}

pub trait ExchangeParameters {
    fn get_id(&self) -> ExchangeId;
}

pub struct Exchange<Exc: ExchangeEssentials + ?Sized> {
    pub parameters: Exc::Parameters,
    pub inner: Exc,
}

impl<Exc: ExchangeEssentials> Debug for Exchange<Exc> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<Exc: ExchangeEssentials> Clone for Exchange<Exc> {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl<Exc: ExchangeEssentials> Default for Exchange<Exc> {
    fn default() -> Self {
        todo!()
    }
}

impl<Exc: ExchangeEssentials> Exchange<Exc> {
    pub async fn new(parameters: Exc::Parameters) -> Self {
        Self {
            parameters: parameters.clone(),
            inner: Exc::new(parameters).await,
        }
    }
}

impl<Exc: ExchangeEssentials + ExchangeInfoRetrieval> Exchange<Exc> {
    pub async fn refresh_market_info(&self) -> Result<Vec<MarketPairHandle>> {
        self.inner.refresh_market_info().await
    }
}

#[async_trait]
pub trait ExchangeEssentials {
    type Parameters: ExchangeParameters + Clone;

    async fn new(parameters: Self::Parameters) -> Self;
}

#[async_trait]
pub trait ExchangeSpec: Unpin {
    type OrderId: Debug + Clone + Serialize + DeserializeOwned;
    type TradeId: Debug + Clone + Serialize + DeserializeOwned;
    type Pagination: Debug + Clone + Serialize + DeserializeOwned;
}

#[async_trait]
pub trait ExchangeMarketData: ExchangeSpec + Sized {
    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse>;
    async fn get_price_ticker(&self, req: &GetPriceTickerRequest) -> Result<Ticker>;
    async fn get_historic_rates(&self, req: &GetHistoricRatesRequest<Self>) -> Result<Vec<Candle>>;
    async fn get_historic_trades(
        &self,
        req: &GetHistoricTradesRequest<Self>,
    ) -> Result<Vec<Trade<Self>>>;
}

#[async_trait]
pub trait ExchangeAccount: ExchangeSpec + Sized {
    async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self>>;
    async fn limit_sell(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self>>;
    async fn market_buy(&self, req: &OpenMarketOrderRequest) -> Result<Order<Self>>;
    async fn market_sell(&self, req: &OpenMarketOrderRequest) -> Result<Order<Self>>;
    async fn cancel_order(&self, req: &CancelOrderRequest<Self>) -> Result<OrderCanceled<Self>>;
    async fn cancel_all_orders(
        &self,
        req: &CancelAllOrdersRequest,
    ) -> Result<Vec<OrderCanceled<Self>>>;
    async fn get_all_open_orders(&self) -> Result<Vec<Order<Self>>>;
    async fn get_order_history(
        &self,
        req: &GetOrderHistoryRequest<Self>,
    ) -> Result<Vec<Order<Self>>>;
    async fn get_trade_history(&self, req: &TradeHistoryRequest<Self>) -> Result<Vec<Trade<Self>>>;
    async fn get_account_balances(
        &self,
        paginator: Option<Paginator<Self>>,
    ) -> Result<Vec<Balance>>;
    async fn get_order(&self, req: &GetOrderRequest<Self>) -> Result<Order<Self>>;
}
