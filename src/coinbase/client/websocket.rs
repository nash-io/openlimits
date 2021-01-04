use async_trait::async_trait;
use std::{collections::HashMap, pin::Pin, task::Poll};

use futures::{
    stream::{SplitStream, Stream},
    SinkExt, StreamExt,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;

use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use crate::{
    coinbase::model::websocket::{
        Channel, CoinbaseSubscription, CoinbaseWebsocketMessage, Subscribe, SubscribeCmd,
    },
    errors::OpenLimitsError,
    shared::Result,
};

use crate::coinbase::model::websocket::ChannelType;
use crate::coinbase::CoinbaseParameters;
use crate::exchange_ws::{ExchangeWs, Subscriptions};
use futures::stream::BoxStream;
use std::sync::Mutex;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

const WS_URL_PROD: &str = "wss://ws-feed.pro.coinbase.com";
const WS_URL_SANDBOX: &str = "wss://ws-feed-public.sandbox.pro.coinbase.com";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct CoinbaseWebsocket {
    pub subscriptions: HashMap<CoinbaseSubscription, SplitStream<WSStream>>,
    pub parameters: CoinbaseParameters,
    disconnection_senders: Mutex<Vec<UnboundedSender<()>>>,
}

impl CoinbaseWebsocket {
    pub fn new(parameters: CoinbaseParameters) -> Self {
        Self {
            subscriptions: Default::default(),
            parameters,
            disconnection_senders: Default::default(),
        }
    }

    pub async fn subscribe_(&mut self, subscription: CoinbaseSubscription) -> Result<()> {
        let (channels, product_ids) = match &subscription {
            CoinbaseSubscription::Level2(product_id) => (
                vec![Channel::Name(ChannelType::Level2)],
                vec![product_id.clone()],
            ),
            CoinbaseSubscription::Heartbeat(product_id) => (
                vec![Channel::Name(ChannelType::Heartbeat)],
                vec![product_id.clone()],
            ),
            _ => unimplemented!(),
        };
        let subscribe = Subscribe {
            _type: SubscribeCmd::Subscribe,
            auth: None,
            channels,
            product_ids,
        };

        let stream = self.connect(subscribe).await?;
        self.subscriptions.insert(subscription, stream);
        Ok(())
    }

    pub async fn connect(&self, subscribe: Subscribe) -> Result<SplitStream<WSStream>> {
        let ws_url = if self.parameters.sandbox {
            WS_URL_SANDBOX
        } else {
            WS_URL_PROD
        };
        let url = url::Url::parse(ws_url).expect("Couldn't parse url.");
        let (ws_stream, _) = connect_async(&url).await?;
        let (mut sink, stream) = ws_stream.split();
        let subscribe = serde_json::to_string(&subscribe)?;

        sink.send(Message::Text(subscribe)).await?;
        Ok(stream)
    }
}

impl Stream for CoinbaseWebsocket {
    type Item = Result<CoinbaseWebsocketMessage>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        for (_sub, stream) in &mut self.subscriptions.iter_mut() {
            if let Poll::Ready(Some(message)) = Pin::new(stream).poll_next(cx) {
                let message = parse_message(message?);
                return Poll::Ready(Some(message));
            }
        }

        std::task::Poll::Pending
    }
}

fn parse_message(ws_message: Message) -> Result<CoinbaseWebsocketMessage> {
    let msg = match ws_message {
        Message::Text(m) => m,
        _ => return Err(OpenLimitsError::SocketError()),
    };
    Ok(serde_json::from_str(&msg)?)
}

#[async_trait]
impl ExchangeWs for CoinbaseWebsocket {
    type InitParams = CoinbaseParameters;
    type Subscription = CoinbaseSubscription;
    type Response = CoinbaseWebsocketMessage;

    async fn new(parameters: Self::InitParams) -> Result<Self> {
        Ok(CoinbaseWebsocket::new(parameters))
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
        subscription: Subscriptions<Self::Subscription>,
    ) -> Result<BoxStream<'static, Result<Self::Response>>> {
        let ws_url = if self.parameters.sandbox {
            WS_URL_SANDBOX
        } else {
            WS_URL_PROD
        };
        let endpoint = url::Url::parse(ws_url).expect("Couldn't parse url.");
        let (ws_stream, _) = connect_async(endpoint).await?;

        let (channels, product_ids) = match &subscription.as_slice()[0] {
            CoinbaseSubscription::Level2(product_id) => (
                vec![Channel::Name(ChannelType::Level2)],
                vec![product_id.clone()],
            ),
            CoinbaseSubscription::Heartbeat(product_id) => (
                vec![Channel::Name(ChannelType::Heartbeat)],
                vec![product_id.clone()],
            ),
            _ => unimplemented!(),
        };
        let subscribe = Subscribe {
            _type: SubscribeCmd::Subscribe,
            auth: None,
            channels,
            product_ids,
        };
        let subscribe = serde_json::to_string(&subscribe)?;
        let (mut sink, stream) = ws_stream.split();
        let (disconnection_sender, mut disconnection_receiver) = unbounded_channel();
        sink.send(Message::Text(subscribe)).await?;
        tokio::spawn(async move {
            if disconnection_receiver.recv().await.is_some() {
                sink.close().await.ok();
            }
        });

        if let Ok(mut senders) = self.disconnection_senders.lock() {
            senders.push(disconnection_sender);
        }
        let s = stream.map(|message| parse_message(message?));

        Ok(s.boxed())
    }
}
