use std::{
    any::Any,
    convert::TryInto,
    pin::Pin,
    task::{Context, Poll},
};

use crate::{
    any_exchange::AnyWsExchange,
    errors::OpenLimitError,
    model::websocket::{OpenLimitsWebSocketMessage, Subscription},
    shared::Result,
};
use async_trait::async_trait;
use derive_more::Constructor;
use futures::{channel::mpsc::channel, future, stream::BoxStream, Stream, StreamExt, TryStream};

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

    pub async fn subscribe_specific<F: Fn(&Result<E::Response>) + Sync + Send + 'static>(
        &self,
        subscription: E::Subscription,
        callback: F,
    ) -> Result<CallbackHandle> {
        self.websocket
            .subscribe_specific(subscription, callback)
            .await
    }
    pub async fn subscribe<F: Fn(&Result<OpenLimitsWebSocketMessage>) + Sync + Send + 'static>(
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
    type Response: TryInto<OpenLimitsWebSocketMessage> + Send + Sync + Clone + Sized + 'static;

    async fn new(params: Self::InitParams) -> Self
    where
        Self: Sized;

    async fn create_stream_specific(
        &self,
        subscriptions: &[Self::Subscription],
    ) -> Result<BoxStream<'static, Result<Self::Response>>>;

    async fn subscribe_specific<F: Fn(&Result<Self::Response>) + Send + 'static>(
        &self,
        subscription: Self::Subscription,
        callback: F,
    ) -> Result<CallbackHandle> {
        let pin = self.create_stream_specific(&[subscription]).await.unwrap();
        let fut = pin.then(move |m| {
            callback(&m);
            future::ready(m)
        });

        let handle = spawn(fut);

        Ok(handle)
    }

    async fn subscribe<F: Fn(&Result<OpenLimitsWebSocketMessage>) + Send + 'static>(
        &self,
        subscription: Subscription,
        callback: F,
    ) -> Result<CallbackHandle> {
        self.subscribe_specific(subscription.into(), move |m| {
            if let Ok(message) = m {
                let r = message
                    .clone()
                    .try_into()
                    .map_err(|_| OpenLimitError::SocketError());
                callback(&r)
            }
        })
        .await
    }
}

pub fn spawn<S>(stream: S) -> CallbackHandle
where
    S: TryStream + Send + Unpin + 'static,
    S::Item: Send,
{
    let (tx, rx) = channel(1);

    tokio::spawn(
        stream
            .map(Ok)
            .skip_while(|_| future::ready(true))
            .forward(tx),
    );

    CallbackHandle { rx: Box::new(rx) }
}

#[derive(Debug)]
pub struct CallbackHandle {
    rx: Box<dyn Any + Send>,
}

pub struct SubscriptionStream<'a, E: ExchangeWs> {
    pub inner_stream: BoxStream<'static, Result<OpenLimitsWebSocketMessage>>,
    pub exchange: &'a E,
}

impl<'a, E: ExchangeWs> SubscriptionStream<'a, E> {
    pub fn new(
        inner_stream: BoxStream<'static, Result<OpenLimitsWebSocketMessage>>,
        exchange: &'a E,
    ) -> Self {
        Self {
            inner_stream,
            exchange,
        }
    }
}

impl<'a, E: ExchangeWs> From<Result<SubscriptionStream<'a, E>>>
    for SubscriptionStream<'a, AnyWsExchange>
{
    fn from(_: Result<SubscriptionStream<'a, E>>) -> Self {
        todo!()
    }
}

impl<'a, E: ExchangeWs> Stream for SubscriptionStream<'a, E> {
    type Item = Result<OpenLimitsWebSocketMessage>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner_stream.as_mut().poll_next(cx)
    }
}
