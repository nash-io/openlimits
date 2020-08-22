use url::Url;

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
        Channel, CoinbaseWebsocketMessage, Subscribe, SubscribeCmd, Subscription,
    },
    shared::Result,
    errors::OpenLimitError
};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct CoinbaseWebsocket {
    pub subscriptions: HashMap<Subscription, SplitStream<WSStream>>,
    pub url: Url,
}

impl CoinbaseWebsocket {
    pub fn new(url: &str) -> Self {
        let url = Url::parse(url).unwrap();
        Self {
            subscriptions: HashMap::new(),
            url,
        }
    }

    pub async fn subscribe(&mut self, subscription: Subscription) -> Result<()> {
        let subscribe = Subscribe {
            _type: SubscribeCmd::Subscribe,
            auth: None,
            channels: subscription
                .channels
                .to_vec()
                .into_iter()
                .map(Channel::Name)
                .collect::<Vec<_>>(),
            product_ids: subscription.product_ids.clone(),
        };

        let stream = self.connect(subscribe).await?;
        self.subscriptions.insert(subscription, stream);
        Ok(())
    }

    pub async fn connect(&self, subscribe: Subscribe) -> Result<SplitStream<WSStream>> {
        let (ws_stream, _) = connect_async(&self.url).await?;
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
        _ => return Err(OpenLimitError::SocketError()),
    };
    Ok(serde_json::from_str(&msg)?)
}
