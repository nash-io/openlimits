use std::{convert::TryFrom, fmt::Display};
use std::sync::Mutex;
use async_trait::async_trait;
use futures::{SinkExt, stream::BoxStream, StreamExt};
use serde::{de, Deserialize, Serialize};
use serde_json::Value;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use openlimits_exchange::errors::OpenLimitsError;
use crate::{
    BinanceParameters,
    model::websocket::{BinanceSubscription, BinanceWebsocketMessage},
};
use openlimits_exchange::{
    model::websocket::OpenLimitsWebSocketMessage,
    model::websocket::Subscription,
    model::websocket::WebSocketResponse,
};
use openlimits_exchange::traits::stream::{ExchangeWs, Subscriptions};
use super::shared::Result;
use openlimits_exchange::exchange::Environment;

const WS_URL_PROD: &str = "wss://stream.binance.com:9443/stream";
const WS_URL_SANDBOX: &str = "wss://testnet.binance.vision/stream";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

/// This struct is used for websocket communications with openlimits-binance openlimits-exchange
pub struct BinanceWebsocket {
    parameters: BinanceParameters,
    disconnection_senders: Mutex<Vec<UnboundedSender<()>>>,
}

#[async_trait]
impl ExchangeWs for BinanceWebsocket {
    type InitParams = BinanceParameters;
    type Subscription = BinanceSubscription;
    type Response = BinanceWebsocketMessage;

    async fn new(parameters: Self::InitParams) -> Result<Self> {
        Ok(BinanceWebsocket {
            parameters,
            disconnection_senders: Default::default(),
        })
    }

    async fn disconnect(&self) {
        if let Ok(mut senders) = self.disconnection_senders.lock() {
            for sender in senders.iter() {
                sender.send(()).ok();
            }
            senders.clear();
        }
    }

    async fn create_stream_specific(
        &self,
        subscriptions: Subscriptions<Self::Subscription>,
    ) -> Result<BoxStream<'static, Result<Self::Response>>> {
        let streams = subscriptions
            .into_iter()
            .map(|bs| bs.to_string())
            .collect::<Vec<String>>()
            .join("/");

        let ws_url = match self.parameters.environment {
            Environment::Sandbox => WS_URL_SANDBOX,
            Environment::Production => WS_URL_PROD,
        };
        let endpoint = url::Url::parse(&format!("{}?streams={}", ws_url, streams.to_lowercase()))
            .map_err(OpenLimitsError::UrlParserError)?;
        let (ws_stream, _) = connect_async(endpoint).await?;

        let (mut sink, stream) = ws_stream.split();
        let (disconnection_sender, mut disconnection_receiver) = unbounded_channel();
        tokio::spawn(async move {
            if disconnection_receiver.recv().await.is_some() {
                sink.close().await.ok();
            }
        });

        if let Ok(mut senders) = self.disconnection_senders.lock() {
            senders.push(disconnection_sender);
        }

        let s = stream.map(|message| match message {
            Ok(msg) => parse_message(msg),
            Err(_) => Err(OpenLimitsError::SocketError()),
        });

        Ok(s.boxed())
    }
}

#[derive(Deserialize)]
struct BinanceWebsocketStream {
    #[serde(rename = "stream")]
    pub name: String,
    pub data: Value,
}

impl<'de> Deserialize<'de> for BinanceWebsocketMessage {
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let stream: BinanceWebsocketStream = BinanceWebsocketStream::deserialize(deserializer)?;

        if stream.name.ends_with("@aggTrade") {
            Ok(BinanceWebsocketMessage::AggregateTrade(
                serde_json::from_value(stream.data).map_err(de::Error::custom)?,
            ))
        } else if stream.name.contains("@trade") {
            Ok(BinanceWebsocketMessage::Trade(
                serde_json::from_value(stream.data).map_err(de::Error::custom)?,
            ))
        } else if stream.name.contains("@kline_") {
            Ok(BinanceWebsocketMessage::Candlestick(
                serde_json::from_value(stream.data).map_err(de::Error::custom)?,
            ))
        } else if stream.name.contains("@ticker") {
            Ok(BinanceWebsocketMessage::Ticker(
                serde_json::from_value(stream.data).map_err(de::Error::custom)?,
            ))
        } else if stream.name.eq("!ticker@arr") {
            Ok(BinanceWebsocketMessage::TickerAll(
                serde_json::from_value(stream.data).map_err(de::Error::custom)?,
            ))
        } else if stream.name.ends_with("@miniTicker") {
            Ok(BinanceWebsocketMessage::MiniTicker(
                serde_json::from_value(stream.data).map_err(de::Error::custom)?,
            ))
        } else if stream.name.ends_with("!miniTicker@arr") {
            Ok(BinanceWebsocketMessage::MiniTickerAll(
                serde_json::from_value(stream.data).map_err(de::Error::custom)?,
            ))
        } else if stream.name.ends_with("@depth") {
            Ok(BinanceWebsocketMessage::Depth(
                serde_json::from_value(stream.data).map_err(de::Error::custom)?,
            ))
        } else if stream.name.contains("@depth") {
            Ok(BinanceWebsocketMessage::OrderBook(
                serde_json::from_value(stream.data).map_err(de::Error::custom)?,
            ))
        } else {
            panic!("Not supported Subscription");
        }
    }
}

impl Display for BinanceSubscription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinanceSubscription::AggregateTrade(ref symbol) => write!(f, "{}@aggTrade", symbol),
            BinanceSubscription::Candlestick(ref symbol, ref interval) => {
                write!(f, "{}@kline_{}", symbol, interval)
            }
            BinanceSubscription::Depth(ref symbol, interval) => match interval {
                None => write!(f, "{}@depth", symbol),
                Some(i) => write!(f, "{}@depth@{}ms", symbol, i),
            },
            BinanceSubscription::MiniTicker(symbol) => write!(f, "{}@miniTicker", symbol),
            BinanceSubscription::MiniTickerAll => write!(f, "!miniTicker@arr"),
            BinanceSubscription::OrderBook(ref symbol, depth) => {
                write!(f, "{}@depth{}", symbol, depth)
            }
            BinanceSubscription::Ticker(ref symbol) => write!(f, "{}@ticker", symbol),
            BinanceSubscription::TickerAll => write!(f, "!ticker@arr"),
            BinanceSubscription::Trade(ref symbol) => write!(f, "{}@trade", symbol),
            BinanceSubscription::UserData(ref key) => write!(f, "{}", key),
        }
    }
}

impl From<Subscription> for BinanceSubscription {
    fn from(subscription: Subscription) -> Self {
        match subscription {
            Subscription::OrderBookUpdates(symbol) => BinanceSubscription::Depth(crate::model::MarketPair::from(symbol).0, None),
            Subscription::Trades(symbol) => BinanceSubscription::Trade(crate::model::MarketPair::from(symbol).0)
        }
    }
}

impl TryFrom<BinanceWebsocketMessage> for WebSocketResponse<BinanceWebsocketMessage> {
    type Error = OpenLimitsError;

    fn try_from(value: BinanceWebsocketMessage) -> Result<Self> {
        match value {
            BinanceWebsocketMessage::Depth(orderbook) => Ok(WebSocketResponse::Generic(
                OpenLimitsWebSocketMessage::OrderBook(orderbook.into()),
            )),
            BinanceWebsocketMessage::Trade(trade) => Ok(WebSocketResponse::Generic(
                OpenLimitsWebSocketMessage::Trades(trade.into()),
            )),
            BinanceWebsocketMessage::Ping => {
                Ok(WebSocketResponse::Generic(OpenLimitsWebSocketMessage::Ping))
            }
            BinanceWebsocketMessage::Close => Err(OpenLimitsError::SocketError()),
            _ => Ok(WebSocketResponse::Raw(value)),
        }
    }
}

fn parse_message(ws_message: Message) -> Result<BinanceWebsocketMessage> {
    let msg = match ws_message {
        Message::Text(m) => m,
        Message::Binary(b) => return Ok(BinanceWebsocketMessage::Binary(b)),
        Message::Pong(..) => return Ok(BinanceWebsocketMessage::Pong),
        Message::Ping(..) => return Ok(BinanceWebsocketMessage::Ping),
        Message::Close(..) => return Ok(BinanceWebsocketMessage::Close),
    };

    serde_json::from_str(&msg).map_err(OpenLimitsError::JsonError)
}
