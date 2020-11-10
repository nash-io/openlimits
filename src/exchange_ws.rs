use std::{
    ops::Deref,
    pin::Pin,
    task::{Context, Poll},
};

use crate::model::websocket::OpenLimitsWebsocketMessage;
use async_trait::async_trait;
use derive_more::Constructor;
use futures::{
    channel::mpsc::channel, channel::mpsc::Receiver, future, Stream, StreamExt, TryStream,
};

use crate::shared::Result;

#[derive(Constructor)]
pub struct OpenLimitsWs<E: ExchangeWs> {
    pub websocket: E,
}

impl<E: ExchangeWs> OpenLimitsWs<E> {
    pub async fn instantiate(params: E::InitParams) -> Self {
        let websocket = E::new(params).await;
        Self { websocket }
    }

    pub async fn subscribe<
        S: Into<E::Subscription> + Clone + Sync + Send,
        F: Fn(&Result<OpenLimitsWebsocketMessage>) + Sync + Send + 'static,
    >(
        &self,
        subscription: S,
        callback: F,
    ) -> Result<CallbackHandle<Result<OpenLimitsWebsocketMessage>>> {
        self.websocket.subscribe(subscription, callback).await
    }

    pub async fn create_stream<S: Into<E::Subscription> + Clone + Sync + Send>(
        &self,
        subscriptions: &[S],
    ) -> Result<SubscriptionStream<'_, E>> {
        self.websocket.create_stream(subscriptions).await
    }
}

#[async_trait]
pub trait ExchangeWs: Sized {
    type InitParams;
    type Subscription;

    async fn new(params: Self::InitParams) -> Self;

    async fn subscribe<
        S: Into<Self::Subscription> + Clone + Sync + Send,
        F: Fn(&Result<OpenLimitsWebsocketMessage>) + Sync + Send + 'static,
    >(
        &self,
        subscription: S,
        callback: F,
    ) -> Result<CallbackHandle<Result<OpenLimitsWebsocketMessage>>>;

    async fn create_stream<'a, S: Into<Self::Subscription> + Clone + Sync + Send>(
        &'a self,
        subscriptions: &[S],
    ) -> Result<SubscriptionStream<'a, Self>>;
}

pub fn spawn<S>(stream: S) -> CallbackHandle<S::Item>
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

    CallbackHandle { rx }
}

#[derive(Debug)]
pub struct CallbackHandle<I> {
    rx: Receiver<I>,
}

impl<I> Deref for CallbackHandle<I> {
    type Target = Receiver<I>;

    fn deref(&self) -> &Self::Target {
        &self.rx
    }
}

pub struct SubscriptionStream<'a, E: ExchangeWs> {
    pub inner_stream:
        Box<dyn Stream<Item = Result<OpenLimitsWebsocketMessage>> + Unpin + Send + 'static>,
    pub exchange: &'a E,
}

impl<'a, E: ExchangeWs> SubscriptionStream<'a, E> {
    pub fn new(
        inner_stream: Box<
            dyn Stream<Item = Result<OpenLimitsWebsocketMessage>> + Unpin + Send + 'static,
        >,
        exchange: &'a E,
    ) -> Self {
        Self {
            inner_stream,
            exchange,
        }
    }
}

impl<'a, E: ExchangeWs> Stream for SubscriptionStream<'a, E> {
    type Item = Result<OpenLimitsWebsocketMessage>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(self.inner_stream.as_mut()).poll_next(cx)
    }
}
