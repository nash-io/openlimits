use crate::{
    binance::model::websocket::{BinanceSubscription, BinanceWebsocketMessage},
    errors::OpenLimitError,
    exchange_ws::ExchangeWs,
    model::websocket::OpenLimitsWebSocketMessage,
    model::websocket::Subscription,
    shared::Result,
};

use async_trait::async_trait;
use futures::{stream::BoxStream, stream::SplitStream, StreamExt};
use serde::{de, Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, convert::TryFrom, fmt::Display};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

const WS_URL: &str = "wss://stream.binance.com:9443/stream";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct BinanceWebsocket {
    pub subscriptions: HashMap<Subscription, SplitStream<WSStream>>,
}

impl BinanceWebsocket {
    pub fn new() -> Self {
        Self {
            subscriptions: HashMap::new(),
        }
    }
}

#[async_trait]
impl ExchangeWs for BinanceWebsocket {
    type InitParams = ();
    type Subscription = BinanceSubscription;
    type Response = BinanceWebsocketMessage;

    async fn new(_: ()) -> Self {
        BinanceWebsocket::new()
    }

    async fn create_stream_specific(
        &self,
        subscriptions: &[Self::Subscription],
    ) -> Result<BoxStream<'static, Result<Self::Response>>> {
        let streams = subscriptions
            .iter()
            .map(|bs| bs.to_string())
            .collect::<Vec<String>>()
            .join("/");

        let endpoint = url::Url::parse(&format!("{}?streams={}", WS_URL, streams)).unwrap();
        let (ws_stream, _) = connect_async(endpoint).await?;

        let s = ws_stream.map(|message| parse_message(message.unwrap()));

        Ok(s.boxed())
    }
}

impl Default for BinanceWebsocket {
    fn default() -> Self {
        Self::new()
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

        if stream.name.eq("!ticker@arr") {
            Ok(BinanceWebsocketMessage::TickerAll(
                serde_json::from_value(stream.data).map_err(de::Error::custom)?,
            ))
        } else if stream.name.contains("@depth10") {
            Ok(BinanceWebsocketMessage::OrderBook(
                serde_json::from_value(stream.data).map_err(de::Error::custom)?,
            ))
        } else if stream.name.contains("@depth") {
            Ok(BinanceWebsocketMessage::Depth(
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
            Subscription::OrderBook(symbol) => BinanceSubscription::Depth(symbol, None),
            Subscription::Trades(symbol) => BinanceSubscription::Trade(symbol),
            _ => panic!("Not implemented"),
        }
    }
}

impl TryFrom<BinanceWebsocketMessage> for OpenLimitsWebSocketMessage {
    type Error = OpenLimitError;

    fn try_from(value: BinanceWebsocketMessage) -> Result<Self> {
        match value {
            BinanceWebsocketMessage::OrderBook(orderbook) => {
                Ok(OpenLimitsWebSocketMessage::OrderBook(orderbook.into()))
            }
            BinanceWebsocketMessage::Ping => Ok(OpenLimitsWebSocketMessage::Ping),
            BinanceWebsocketMessage::Close => Err(OpenLimitError::SocketError()),
            _ => Err(OpenLimitError::WebSocketMessageNotSupported()),
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

    serde_json::from_str(&msg).map_err(OpenLimitError::JsonError)
}
