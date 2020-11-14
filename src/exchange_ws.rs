use std::{
    any::Any,
    convert::{TryFrom, TryInto},
};

use crate::{
    errors::OpenLimitError,
    model::websocket::WebSocketResponse,
    model::websocket::{OpenLimitsWebSocketMessage, Subscription},
    shared::Result,
};
use async_trait::async_trait;
use derive_more::Constructor;
use futures::{channel::mpsc::channel, future, stream::BoxStream, StreamExt};

#[derive(Constructor)]
pub struct OpenLimitsWs<E: ExchangeWs> {
    pub websocket: E,
}

impl<E: ExchangeWs> OpenLimitsWs<E> {
    pub async fn instantiate(params: E::InitParams) -> Self {
        let websocket = E::new(params).await;
        Self { websocket }
    }

    pub async fn create_stream_specific(
        &self,
        subscriptions: &[E::Subscription],
    ) -> Result<BoxStream<'static, Result<E::Response>>> {
        self.websocket.create_stream_specific(subscriptions).await
    }

    pub async fn subscribe<
        F: Fn(&Result<WebSocketResponse<E::Response>>) + Sync + Send + 'static,
    >(
        &self,
        subscription: Subscription,
        callback: F,
    ) -> Result<CallbackHandle> {
        self.websocket.subscribe(subscription, callback).await
    }
}

#[async_trait]
pub trait ExchangeWs: Send + Sync {
    type InitParams;
    type Subscription: From<Subscription> + Send + Sync + Sized;
    type Response: TryInto<WebSocketResponse<Self::Response>>
        + Send
        + Sync
        + Clone
        + Sized
        + 'static;

    async fn new(params: Self::InitParams) -> Self;

    async fn create_stream_specific(
        &self,
        subscriptions: &[Self::Subscription],
    ) -> Result<BoxStream<'static, Result<Self::Response>>>;

    async fn subscribe<
        S: Into<Self::Subscription> + Send,
        F: Fn(&Result<WebSocketResponse<Self::Response>>) + Send + 'static,
    >(
        &self,
        subscription: S,
        callback: F,
    ) -> Result<CallbackHandle> {
        let stream = self
            .create_stream_specific(&[subscription.into()])
            .await
            .unwrap();

        let fut = stream.then(move |m| match m {
            Ok(message) => {
                let r = message
                    .try_into()
                    .map_err(|_| OpenLimitError::SocketError());
                callback(&r);
                future::ready(r)
            }
            Err(err) => future::ready(Err(err)),
        });

        let (tx, rx) = channel(1);

        tokio::spawn(fut.map(Ok).skip_while(|_| future::ready(true)).forward(tx));

        Ok(CallbackHandle { rx: Box::new(rx) })
    }
}

#[derive(Debug)]
pub struct CallbackHandle {
    rx: Box<dyn Any + Send>,
}

impl TryFrom<OpenLimitsWebSocketMessage> for WebSocketResponse<OpenLimitsWebSocketMessage> {
    type Error = OpenLimitError;

    fn try_from(value: OpenLimitsWebSocketMessage) -> Result<Self> {
        todo!()
    }
}
