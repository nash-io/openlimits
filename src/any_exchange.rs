//! In some contexts, such as bindings in other languages (e.g., Python via pyo3), it is not possible to use trait
//! constraints on generics. This module provides an enum wrapper type for all openlimits exchanges that code can
//! use to operate over any openlimits-supported exchange without generics

use std::convert::TryFrom;

use crate::exchange_info::{ExchangeInfoRetrieval, MarketPair, MarketPairHandle};
use crate::exchange_ws::{ExchangeWs, OpenLimitsWs, Subscriptions};
use crate::nash::{Nash, NashParameters, NashWebsocket};
use crate::{
    binance::{Binance, BinanceParameters, BinanceWebsocket},
    model::websocket::OpenLimitsWebSocketMessage,
};
use crate::{
    exchange::{Exchange, ExchangeAccount, ExchangeMarketData},
    model::websocket::WebSocketResponse,
};
use crate::{
    model::{
        websocket::Subscription, Balance, CancelAllOrdersRequest, CancelOrderRequest, Candle,
        GetHistoricRatesRequest, GetHistoricTradesRequest, GetOrderHistoryRequest, GetOrderRequest,
        GetPriceTickerRequest, OpenLimitOrderRequest, OpenMarketOrderRequest, Order,
        OrderBookRequest, OrderBookResponse, OrderCanceled, Paginator, Ticker, Trade,
        TradeHistoryRequest,
    },
    shared::Result,
};
use async_trait::async_trait;
use futures::stream::{BoxStream, StreamExt};

#[derive(Clone)]
pub enum InitAnyExchange {
    Nash(NashParameters),
    Binance(BinanceParameters),
}

pub enum AnyExchange {
    Nash(Nash),
    Binance(Binance),
}

#[async_trait]
impl Exchange for AnyExchange {
    type InitParams = InitAnyExchange;
    type InnerClient = ();
    async fn new(params: InitAnyExchange) -> Self {
        match params {
            InitAnyExchange::Nash(params) => Nash::new(params).await.into(),
            InitAnyExchange::Binance(params) => Binance::new(params).await.into(),
        }
    }
    // not particularly useful to access the inner client with this type. could wrap the inner
    // client reference in an enum, but that would introduce lifetimes all the way down due to
    // https://users.rust-lang.org/t/how-to-specify-lifetime-for-associated-type/5736
    fn inner_client(&self) -> Option<&Self::InnerClient> {
        None
    }
}

#[async_trait]
impl ExchangeInfoRetrieval for AnyExchange {
    async fn get_pair(&self, name: &str) -> Result<MarketPairHandle> {
        match self {
            Self::Nash(nash) => nash.get_pair(name).await,
            Self::Binance(binance) => binance.get_pair(name).await,
        }
    }
    async fn retrieve_pairs(&self) -> Result<Vec<MarketPair>> {
        match self {
            Self::Nash(nash) => nash.retrieve_pairs().await,
            Self::Binance(binance) => binance.retrieve_pairs().await,
        }
    }
    async fn refresh_market_info(&self) -> Result<Vec<MarketPairHandle>> {
        match self {
            Self::Nash(nash) => nash.refresh_market_info().await,
            Self::Binance(binance) => binance.refresh_market_info().await,
        }
    }
}

#[async_trait]
impl ExchangeAccount for AnyExchange {
    async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order> {
        match self {
            Self::Nash(nash) => nash.limit_buy(req).await,
            Self::Binance(binance) => binance.limit_buy(req).await,
        }
    }
    async fn limit_sell(&self, req: &OpenLimitOrderRequest) -> Result<Order> {
        match self {
            Self::Nash(nash) => nash.limit_sell(req).await,
            Self::Binance(binance) => binance.limit_sell(req).await,
        }
    }
    async fn market_buy(&self, req: &OpenMarketOrderRequest) -> Result<Order> {
        match self {
            Self::Nash(nash) => nash.market_buy(req).await,
            Self::Binance(binance) => binance.market_buy(req).await,
        }
    }
    async fn market_sell(&self, req: &OpenMarketOrderRequest) -> Result<Order> {
        match self {
            Self::Nash(nash) => nash.market_sell(req).await,
            Self::Binance(binance) => binance.market_sell(req).await,
        }
    }
    async fn cancel_order(&self, req: &CancelOrderRequest) -> Result<OrderCanceled> {
        match self {
            Self::Nash(nash) => nash.cancel_order(req).await,
            Self::Binance(binance) => binance.cancel_order(req).await,
        }
    }
    async fn cancel_all_orders(&self, req: &CancelAllOrdersRequest) -> Result<Vec<OrderCanceled>> {
        match self {
            Self::Nash(nash) => nash.cancel_all_orders(req).await,
            Self::Binance(binance) => binance.cancel_all_orders(req).await,
        }
    }
    async fn get_all_open_orders(&self) -> Result<Vec<Order>> {
        match self {
            Self::Nash(nash) => nash.get_all_open_orders().await,
            Self::Binance(binance) => binance.get_all_open_orders().await,
        }
    }
    async fn get_order_history(&self, req: &GetOrderHistoryRequest) -> Result<Vec<Order>> {
        match self {
            Self::Nash(nash) => nash.get_order_history(req).await,
            Self::Binance(binance) => binance.get_order_history(req).await,
        }
    }
    async fn get_trade_history(&self, req: &TradeHistoryRequest) -> Result<Vec<Trade>> {
        match self {
            Self::Nash(nash) => nash.get_trade_history(req).await,
            Self::Binance(binance) => binance.get_trade_history(req).await,
        }
    }
    async fn get_account_balances(&self, paginator: Option<Paginator>) -> Result<Vec<Balance>> {
        match self {
            Self::Nash(nash) => nash.get_account_balances(paginator).await,
            Self::Binance(binance) => binance.get_account_balances(paginator).await,
        }
    }
    async fn get_order(&self, req: &GetOrderRequest) -> Result<Order> {
        match self {
            Self::Nash(nash) => nash.get_order(req).await,
            Self::Binance(binance) => binance.get_order(req).await,
        }
    }
}

#[async_trait]
impl ExchangeMarketData for AnyExchange {
    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse> {
        match self {
            Self::Nash(nash) => nash.order_book(req).await,
            Self::Binance(binance) => binance.order_book(req).await,
        }
    }
    async fn get_price_ticker(&self, req: &GetPriceTickerRequest) -> Result<Ticker> {
        match self {
            Self::Nash(nash) => nash.get_price_ticker(req).await,
            Self::Binance(binance) => binance.get_price_ticker(req).await,
        }
    }
    async fn get_historic_rates(&self, req: &GetHistoricRatesRequest) -> Result<Vec<Candle>> {
        match self {
            Self::Nash(nash) => nash.get_historic_rates(req).await,
            Self::Binance(binance) => binance.get_historic_rates(req).await,
        }
    }
    async fn get_historic_trades(&self, req: &GetHistoricTradesRequest) -> Result<Vec<Trade>> {
        match self {
            Self::Nash(nash) => nash.get_historic_trades(req).await,
            Self::Binance(binance) => binance.get_historic_trades(req).await,
        }
    }
}

pub enum AnyWsExchange {
    Nash(OpenLimitsWs<NashWebsocket>),
    Binance(OpenLimitsWs<BinanceWebsocket>),
}

#[async_trait]
impl ExchangeWs for AnyWsExchange {
    type InitParams = InitAnyExchange;
    type Subscription = Subscription;
    type Response = OpenLimitsWebSocketMessage;

    async fn new(params: Self::InitParams) -> Self {
        match params {
            InitAnyExchange::Nash(params) => {
                OpenLimitsWs::<NashWebsocket>::instantiate(params).await.into()
            }
            InitAnyExchange::Binance(params) => {
                OpenLimitsWs::<BinanceWebsocket>::instantiate(params)
                    .await
                    .into()
            }
        }
    }

    async fn create_stream_specific(
        &self,
        subscriptions: Subscriptions<Self::Subscription>,
    ) -> Result<BoxStream<'static, Result<Self::Response>>> {
        let s = match self {
            Self::Nash(nash) => nash
                .create_stream_specific(subscriptions.as_slice().into())
                .await?
                .map(|r| WebSocketResponse::try_from(r.unwrap()))
                .map(|r| {
                    r.map(|resp| match resp {
                        WebSocketResponse::Generic(generic) => generic,
                        WebSocketResponse::Raw(_) => panic!("Should never happen"),
                    })
                })
                .boxed(),
            Self::Binance(binance) => binance
                .create_stream_specific(subscriptions.as_slice().into())
                .await?
                .map(|r| WebSocketResponse::try_from(r.unwrap()))
                .map(|r| {
                    r.map(|resp| match resp {
                        WebSocketResponse::Generic(generic) => generic,
                        WebSocketResponse::Raw(_) => panic!("Should never happen"),
                    })
                })
                .boxed(),
        };
        Ok(s)
    }
}

impl From<Nash> for AnyExchange {
    fn from(nash: Nash) -> Self {
        Self::Nash(nash)
    }
}

impl From<Binance> for AnyExchange {
    fn from(binance: Binance) -> Self {
        Self::Binance(binance)
    }
}

impl From<OpenLimitsWs<NashWebsocket>> for AnyWsExchange {
    fn from(nash: OpenLimitsWs<NashWebsocket>) -> Self {
        Self::Nash(nash)
    }
}

impl From<OpenLimitsWs<BinanceWebsocket>> for AnyWsExchange {
    fn from(binance: OpenLimitsWs<BinanceWebsocket>) -> Self {
        Self::Binance(binance)
    }
}
