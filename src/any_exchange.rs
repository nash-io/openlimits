//! In some contexts, such as bindings in other languages (e.g., Python via pyo3) it is not possible to use trait
//! constraints on generics. This module provides an enum wrapper type for all openlimits exchanges that code can 
//! use to operate over any openlimits exchange without generics

use crate::{
    model::{
        Balance, CancelAllOrdersRequest, CancelOrderRequest, Candle, GetHistoricRatesRequest,
        GetHistoricTradesRequest, GetOrderHistoryRequest, GetOrderRequest, GetPriceTickerRequest,
        OpenLimitOrderRequest, OpenMarketOrderRequest, Order, OrderBookRequest, OrderBookResponse,
        OrderCanceled, Paginator, Ticker, Trade, TradeHistoryRequest,
    },
    shared::Result,
};
use crate::nash::{NashParameters, Nash, NashStream, Client as NashBaseClient};
use crate::binance::{BinanceParameters, Binance, BinanceWebsocket, BaseClient as BinanceBaseClient};
use crate::exchange::{Exchange, ExchangeAccount, ExchangeMarketData};
use crate::exchange_ws::{ExchangeWs, OpenLimitsWs};
use crate::exchange_info::{ExchangeInfo, ExchangeInfoRetrieval};
use async_trait::async_trait;



pub enum AnyCredential {
    Nash(NashParameters),
    Binance(BinanceParameters)
}

pub enum OpenlimitsAnyExchange {
    Nash(Nash),
    Binance(Binance),
}

#[async_trait]
impl Exchange for OpenlimitsAnyExchange {
    type InitParams = AnyCredential;
    type InnerClient = ();
    async fn new(params: AnyCredential) -> Self {
        match params {
            AnyCredential::Nash(params) => Nash::new(params).await.into(),
            AnyCredential::Binance(params) => Binance::new(params).await.into()
        }
    }
    // not particularly useful to access the inner client with this type. could wrap the inner
    // client reference in an enum, but that would introduce lifetimes all the way down due to
    // https://users.rust-lang.org/t/how-to-specify-lifetime-for-associated-type/5736 
    fn inner_client(&self) -> Option<&Self::InnerClient> { None }
}

#[async_trait]
impl ExchangeAccount for OpenlimitsAnyExchange {
    async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order> {
        match self {
            Self::Nash(nash) => nash.limit_buy(req).await,
            Self::Binance(binance) => binance.limit_buy(req).await
        }
    }
    async fn limit_sell(&self, req: &OpenLimitOrderRequest) -> Result<Order> {
        match self {
            Self::Nash(nash) => nash.limit_sell(req).await,
            Self::Binance(binance) => binance.limit_sell(req).await
        }
    }
    async fn market_buy(&self, req: &OpenMarketOrderRequest) -> Result<Order> {
        match self {
            Self::Nash(nash) => nash.market_buy(req).await,
            Self::Binance(binance) => binance.market_buy(req).await
        }
    }
    async fn market_sell(&self, req: &OpenMarketOrderRequest) -> Result<Order> {
        match self {
            Self::Nash(nash) => nash.market_sell(req).await,
            Self::Binance(binance) => binance.market_sell(req).await
        }
    }
    async fn cancel_order(&self, req: &CancelOrderRequest) -> Result<OrderCanceled> {
        match self {
            Self::Nash(nash) => nash.cancel_order(req).await,
            Self::Binance(binance) => binance.cancel_order(req).await
        }
    }
    async fn cancel_all_orders(&self, req: &CancelAllOrdersRequest) -> Result<Vec<OrderCanceled>> {
        match self {
            Self::Nash(nash) => nash.cancel_all_orders(req).await,
            Self::Binance(binance) => binance.cancel_all_orders(req).await
        }
    }
    async fn get_all_open_orders(&self) -> Result<Vec<Order>> {
        match self {
            Self::Nash(nash) => nash.get_all_open_orders().await,
            Self::Binance(binance) => binance.get_all_open_orders().await
        }
    }
    async fn get_order_history(&self, req: &GetOrderHistoryRequest) -> Result<Vec<Order>> {
        match self {
            Self::Nash(nash) => nash.get_order_history(req).await,
            Self::Binance(binance) => binance.get_order_history(req).await
        }
    }
    async fn get_trade_history(&self, req: &TradeHistoryRequest) -> Result<Vec<Trade>> {
        match self {
            Self::Nash(nash) => nash.get_trade_history(req).await,
            Self::Binance(binance) => binance.get_trade_history(req).await
        }
    }
    async fn get_account_balances(&self, paginator: Option<Paginator>) -> Result<Vec<Balance>> {
        match self {
            Self::Nash(nash) => nash.get_account_balances(paginator).await,
            Self::Binance(binance) => binance.get_account_balances(paginator).await
        }
    }
    async fn get_order(&self, req: &GetOrderRequest) -> Result<Order> {
        match self {
            Self::Nash(nash) => nash.get_order(req).await,
            Self::Binance(binance) => binance.get_order(req).await
        }
    }
}

pub enum OpenLimitsAnyWs {
    Nash(OpenLimitsWs<NashStream>),
    Binance(OpenLimitsWs<BinanceWebsocket>)
}

impl From<Nash> for OpenlimitsAnyExchange {
    fn from(nash: Nash) -> Self {
        Self::Nash(nash)
    }
}

impl From<Binance> for OpenlimitsAnyExchange {
    fn from(binance: Binance) -> Self {
        Self::Binance(binance)
    }
}