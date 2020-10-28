use std::fmt::Display;

use crate::{
    binance::{
        model::websocket::{BinanceSubscription, BinanceWebsocketMessage},
        Binance,
    },
    errors::OpenLimitError,
    exchange_ws::ExchangeStreams,
    exchange_ws::WebSocketStream,
    model::websocket::OpenLimitsWebsocketMessage,
    model::websocket::Subscription,
    shared::Result,
};
use async_trait::async_trait;
use futures::StreamExt;
use serde::{de, Deserialize};
use serde_json::Value;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

const WS_URL: &str = "wss://stream.binance.com:9443/stream";

#[async_trait]
impl<'a> ExchangeStreams<'a> for Binance {
    async fn new_stream(
        &'a self,
        subscriptions: &[Subscription],
    ) -> Result<WebSocketStream<'a, Self>> {
        let streams = subscriptions
            .iter()
            .map(|s| BinanceSubscription::from(s.clone()))
            .map(|bs| bs.to_string())
            .collect::<Vec<String>>()
            .join("/");

        let endpoint = url::Url::parse(&format!("{}?streams={}", WS_URL, streams)).unwrap();
        let (ws_stream, _) = connect_async(endpoint).await?;

        let s = ws_stream
            .map(|message| parse_message(message.unwrap()))
            .map(|message| OpenLimitsWebsocketMessage::from(message.unwrap()));

        Ok(WebSocketStream::new(Box::new(s), self))
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

        if stream.name.contains("@depth10") {
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
    fn from(sub: Subscription) -> Self {
        match sub {
            Subscription::OrderBook(symbol) => BinanceSubscription::OrderBook(symbol, 10),
            _ => panic!("Not supported Subscription"),
        }
    }
}

impl From<BinanceWebsocketMessage> for OpenLimitsWebsocketMessage {
    fn from(message: BinanceWebsocketMessage) -> Self {
        match message {
            BinanceWebsocketMessage::Ping => OpenLimitsWebsocketMessage::Ping,
            BinanceWebsocketMessage::OrderBook(orderbook) => {
                OpenLimitsWebsocketMessage::OrderBook(orderbook.into())
            }
            _ => panic!("Not supported Message"),
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
