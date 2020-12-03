use std::{
    any::Any,
    convert::{TryFrom, TryInto},
    slice,
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
        subscriptions: Subscriptions<E::Subscription>,
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

    pub async fn create_stream<S: Into<E::Subscription> + Clone + Send + Sync>(
        &self,
        subscriptions: &[S],
    ) -> Result<BoxStream<'static, Result<WebSocketResponse<E::Response>>>> {
        self.websocket.create_stream(subscriptions).await
    }
}

#[async_trait]
pub trait ExchangeWs: Send + Sync {
    type InitParams;
    type Subscription: From<Subscription> + Send + Sync + Sized;
    type Response: TryInto<WebSocketResponse<Self::Response>, Error = OpenLimitError>
        + Send
        + Sync
        + Clone
        + Sized
        + 'static;

    async fn new(params: Self::InitParams) -> Self;

    async fn create_stream_specific(
        &self,
        subscriptions: Subscriptions<Self::Subscription>,
    ) -> Result<BoxStream<'static, Result<Self::Response>>>;

    async fn subscribe<
        S: Into<Self::Subscription> + Sync + Send + Clone,
        F: FnMut(&Result<WebSocketResponse<Self::Response>>) + Send + 'static,
    >(
        &self,
        subscription: S,
        mut callback: F,
    ) -> Result<CallbackHandle> {
        let s = slice::from_ref(&subscription);
        let stream = self.create_stream_specific(s.into()).await?;

        let fut = stream.then(move |m| match m {
            Ok(message) => {
                let r = message.try_into();
                callback(&r);
                future::ready(r)
            }
            Err(err) => future::ready(Err(err)),
        });

        let (tx, rx) = channel(1);

        tokio::spawn(fut.map(Ok).skip_while(|_| future::ready(true)).forward(tx));

        Ok(CallbackHandle { rx: Box::new(rx) })
    }

    async fn create_stream<S: Into<Self::Subscription> + Clone + Send + Sync>(
        &self,
        subscriptions: &[S],
    ) -> Result<BoxStream<'static, Result<WebSocketResponse<Self::Response>>>> {
        let stream = self
            .create_stream_specific(subscriptions.into())
            .await?
            .map(|r| r?.try_into())
            .boxed();

        Ok(stream)
    }
}

#[derive(Debug)]
pub struct CallbackHandle {
    rx: Box<dyn Any + Send>,
}

impl TryFrom<OpenLimitsWebSocketMessage> for WebSocketResponse<OpenLimitsWebSocketMessage> {
    type Error = OpenLimitError;

    fn try_from(value: OpenLimitsWebSocketMessage) -> Result<Self> {
        Ok(WebSocketResponse::Generic(value))
    }
}

pub struct Subscriptions<T: From<Subscription>> {
    inner: Vec<T>,
}

impl<T: From<Subscription>> Subscriptions<T> {
    pub fn as_slice(&self) -> &[T] {
        &self.inner[..]
    }
}

impl<T: From<Subscription>> IntoIterator for Subscriptions<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<T: From<Subscription>, U: Into<T> + Clone> From<&[U]> for Subscriptions<T> {
    fn from(s: &[U]) -> Self {
        let v = s.iter().cloned().map(U::into).collect::<Vec<_>>();

        Subscriptions { inner: v }
    }
}
