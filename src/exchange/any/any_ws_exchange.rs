use std::convert::TryFrom;
use async_trait::async_trait;
use futures::stream::BoxStream;
use futures::stream::StreamExt;
use crate::exchange::coinbase::client::websocket::CoinbaseWebsocket;
use crate::exchange::traits::stream::ExchangeWs;
use crate::exchange::traits::stream::OpenLimitsWs;
use crate::exchange::traits::stream::Subscriptions;
use crate::exchange::nash::NashWebsocket;
use crate::exchange::binance::BinanceWebsocket;
use crate::model::websocket::OpenLimitsWebSocketMessage;
use crate::model::websocket::WebSocketResponse;
use crate::model::websocket::Subscription;
use super::shared::Result;
use super::InitAnyExchange;

/// Websockets clients, this enum provides a websocket communication with the exchanges
pub enum AnyWsExchange {
    Nash(OpenLimitsWs<NashWebsocket>),
    Binance(OpenLimitsWs<BinanceWebsocket>),
    Coinbase(OpenLimitsWs<CoinbaseWebsocket>),
}

#[async_trait]
impl ExchangeWs for AnyWsExchange {
    type InitParams = InitAnyExchange;
    type Subscription = Subscription;
    type Response = OpenLimitsWebSocketMessage;

    async fn new(params: Self::InitParams) -> Result<Self> {
        match params {
            InitAnyExchange::Nash(params) => OpenLimitsWs::<NashWebsocket>::instantiate(params)
                .await
                .map(|exchange| exchange.into()),
            InitAnyExchange::Binance(params) => {
                OpenLimitsWs::<BinanceWebsocket>::instantiate(params)
                    .await
                    .map(|exchange| exchange.into())
            }
            InitAnyExchange::Coinbase(params) => {
                OpenLimitsWs::<CoinbaseWebsocket>::instantiate(params)
                    .await
                    .map(|exchange| exchange.into())
            }
        }
    }

    async fn disconnect(&self) {
        match self {
            Self::Nash(exchange) => exchange.disconnect().await,
            Self::Coinbase(exchange) => exchange.disconnect().await,
            Self::Binance(exchange) => exchange.disconnect().await,
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
                .map(|r| {
                    WebSocketResponse::try_from(r.expect(
                        "Couldn't convert WebSocketResponse from SubscriptionResponseWrapper.",
                    ))
                })
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
                .map(|r| {
                    WebSocketResponse::try_from(r.expect(
                        "Couldn't convert WebSocketResponse from SubscriptionResponseWrapper.",
                    ))
                })
                .map(|r| {
                    r.map(|resp| match resp {
                        WebSocketResponse::Generic(generic) => generic,
                        WebSocketResponse::Raw(_) => panic!("Should never happen"),
                    })
                })
                .boxed(),
            Self::Coinbase(coinbase) => coinbase
                .create_stream_specific(subscriptions.as_slice().into())
                .await?
                .map(|r| {
                    WebSocketResponse::try_from(r.expect(
                        "Couldn't convert WebSocketResponse from SubscriptionResponseWrapper.",
                    ))
                })
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

impl From<OpenLimitsWs<CoinbaseWebsocket>> for AnyWsExchange {
    fn from(coinbase: OpenLimitsWs<CoinbaseWebsocket>) -> Self {
        Self::Coinbase(coinbase)
    }
}