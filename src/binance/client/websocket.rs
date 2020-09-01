use crate::{
    binance::model::websocket::{
        AccountUpdate, BinanceWebsocketMessage, Subscription, UserOrderUpdate,
    },
    shared::Result,
};

use std::{collections::HashMap, pin::Pin, task::Poll};

use futures::{
    stream::{SplitSink, SplitStream, Stream},
    StreamExt,
    Sink,
    ready,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};

const WS_URL: &str = "wss://stream.binance.com:9443/ws";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct BinanceWebsocket {
    pub subscriptions: HashMap<Subscription, (bool, (SplitSink<WSStream, Message>, SplitStream<WSStream>))>,
}

impl BinanceWebsocket {
    pub fn new() -> Self {
        Self {
            subscriptions: HashMap::new(),
        }
    }

    pub async fn subscribe(&mut self, subscription: Subscription) -> Result<()> {
        let sub = match &subscription {
            Subscription::AggregateTrade(ref symbol) => format!("{}@aggTrade", symbol),
            Subscription::Candlestick(ref symbol, ref interval) => {
                format!("{}@kline_{}", symbol, interval)
            }
            Subscription::Depth(ref symbol, interval) => match interval {
                None => format!("{}@depth", symbol),
                Some(i) => format!("{}@depth@{}ms", symbol, i),
            },
            Subscription::MiniTicker(symbol) => format!("{}@miniTicker", symbol),
            Subscription::MiniTickerAll => "!miniTicker@arr".to_string(),
            Subscription::OrderBook(ref symbol, depth) => format!("{}@depth{}", symbol, depth),
            Subscription::Ticker(ref symbol) => format!("{}@ticker", symbol),
            Subscription::TickerAll => "!ticker@arr".to_string(),
            Subscription::Trade(ref symbol) => format!("{}@trade", symbol),
            Subscription::UserData(ref key) => key.clone(),
        };

        let endpoint = url::Url::parse(&format!("{}/{}", WS_URL, sub)).unwrap();
        let (ws_stream, _) = connect_async(endpoint).await?;

        self.subscriptions.insert(subscription, (false, ws_stream.split()));

        Ok(())
    }
}

impl Default for BinanceWebsocket {
    fn default() -> Self {
        Self::new()
    }
}

impl Stream for BinanceWebsocket {
    type Item = Result<BinanceWebsocketMessage>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        for (sub, (pong, (sink, stream))) in &mut self.subscriptions.iter_mut() {
            if *pong {
                // Send back a pong frame.

                let mut sink = Pin::new(sink);
                let message = Message::Pong(b"pong frame".to_vec());

                match sink.as_mut().poll_ready(cx)? {
                    Poll::Ready(()) => sink.as_mut().start_send(message)?,
                    Poll::Pending => {
                        // Wait until sink is ready.
                        return Poll::Pending;
                    }
                }

                ready!(sink.poll_flush(cx))?;

                // We now have successfully sent the pong.
                *pong = false;
            }
            
            if !*pong {
                if let Poll::Ready(Some(message)) = Pin::new(stream).poll_next(cx) {
                    let m = parse_message(sub.clone(), message?);
                    
                    if let Ok(BinanceWebsocketMessage::Ping) = m {
                        // Ping frame received, we need to send back a pong later.
                        *pong = true;
                    }

                    return Poll::Ready(Some(m));
                }
            }
        }

        std::task::Poll::Pending
    }
}

fn parse_message(sub: Subscription, ws_message: Message) -> Result<BinanceWebsocketMessage> {
    let msg = match ws_message {
        Message::Text(m) => m,
        Message::Binary(b) => return Ok(BinanceWebsocketMessage::Binary(b)),
        Message::Pong(..) => return Ok(BinanceWebsocketMessage::Pong),
        Message::Ping(..) => return Ok(BinanceWebsocketMessage::Ping),
        Message::Close(..) => return Ok(BinanceWebsocketMessage::Close),
    };

    let message = match sub {
        Subscription::AggregateTrade(..) => {
            BinanceWebsocketMessage::AggregateTrade(serde_json::from_str(&msg)?)
        }
        Subscription::Candlestick(..) => {
            BinanceWebsocketMessage::Candlestick(serde_json::from_str(&msg)?)
        }
        Subscription::Depth(..) => BinanceWebsocketMessage::Depth(serde_json::from_str(&msg)?),
        Subscription::MiniTicker(..) => {
            BinanceWebsocketMessage::MiniTicker(serde_json::from_str(&msg)?)
        }
        Subscription::MiniTickerAll => {
            BinanceWebsocketMessage::MiniTickerAll(serde_json::from_str(&msg)?)
        }
        Subscription::OrderBook(..) => {
            BinanceWebsocketMessage::OrderBook(serde_json::from_str(&msg)?)
        }
        Subscription::Ticker(..) => BinanceWebsocketMessage::Ticker(serde_json::from_str(&msg)?),
        Subscription::TickerAll => BinanceWebsocketMessage::TickerAll(serde_json::from_str(&msg)?),
        Subscription::Trade(..) => BinanceWebsocketMessage::Trade(serde_json::from_str(&msg)?),
        Subscription::UserData(..) => {
            let msg: Either<AccountUpdate, UserOrderUpdate> = serde_json::from_str(&msg)?;
            match msg {
                Either::Left(m) => BinanceWebsocketMessage::UserAccountUpdate(m),
                Either::Right(m) => BinanceWebsocketMessage::UserOrderUpdate(m),
            }
        }
    };

    Ok(message)
}
