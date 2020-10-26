use std::fmt::Debug;

use async_trait::async_trait;
// use serde::{de::DeserializeOwned, Serialize};

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

#[derive(Clone, Debug)]
pub struct Exchange<Exc: ExchangeEssentials + ?Sized> {
    pub parameters: Exc::Parameters,
    pub inner: Exc,
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
pub trait ExchangeMarketData {
    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse>;
    async fn get_price_ticker(&self, req: &GetPriceTickerRequest) -> Result<Ticker>;
    async fn get_historic_rates(&self, req: &GetHistoricRatesRequest) -> Result<Vec<Candle>>;
    async fn get_historic_trades(&self, req: &GetHistoricTradesRequest) -> Result<Vec<Trade>>;
}

#[async_trait]
pub trait ExchangeAccount {
    async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order>;
    async fn limit_sell(&self, req: &OpenLimitOrderRequest) -> Result<Order>;
    async fn market_buy(&self, req: &OpenMarketOrderRequest) -> Result<Order>;
    async fn market_sell(&self, req: &OpenMarketOrderRequest) -> Result<Order>;
    async fn cancel_order(&self, req: &CancelOrderRequest) -> Result<OrderCanceled>;
    async fn cancel_all_orders(&self, req: &CancelAllOrdersRequest) -> Result<Vec<OrderCanceled>>;
    async fn get_all_open_orders(&self) -> Result<Vec<Order>>;
    async fn get_order_history(&self, req: &GetOrderHistoryRequest) -> Result<Vec<Order>>;
    async fn get_trade_history(&self, req: &TradeHistoryRequest) -> Result<Vec<Trade>>;
    async fn get_account_balances(&self, paginator: Option<Paginator>) -> Result<Vec<Balance>>;
    async fn get_order(&self, req: &GetOrderRequest) -> Result<Order>;
}
