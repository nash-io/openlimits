use crate::{
    exchange::Exchange,
    model::websocket::{OpenLimitsWebsocketMessage, Subscription},
};
use async_trait::async_trait;
use futures::stream::Stream;

use crate::shared::Result;
use std::{pin::Pin, task::Context, task::Poll};

#[async_trait]
pub trait ExchangeStreams<'a>: Exchange + Sized {
    async fn new_stream(
        &'a self,
        subscriptions: &[Subscription],
    ) -> Result<WebSocketStream<'a, Self>>;
}

#[async_trait]
pub trait ExchangeMutableStreams<'a>: ExchangeStreams<'a> {
    async fn subscribe(
        &self,
        stream: &dyn Stream<Item = OpenLimitsWebsocketMessage>,
        subscription: &Subscription,
    ) -> Result<()>;
    async fn unsubscribe(
        &self,
        stream: &dyn Stream<Item = OpenLimitsWebsocketMessage>,
        subscription: &Subscription,
    ) -> Result<()>;
}

pub struct WebSocketStream<'a, E: ExchangeStreams<'a>> {
    inner_stream: Box<dyn Stream<Item = OpenLimitsWebsocketMessage> + Unpin>,
    exchange: &'a E,
}

impl<'a, E: ExchangeStreams<'a>> WebSocketStream<'a, E> {
    pub fn new(
        inner_stream: Box<dyn Stream<Item = OpenLimitsWebsocketMessage> + Unpin>,
        exchange: &'a E,
    ) -> Self {
        Self {
            inner_stream,
            exchange,
        }
    }
}

impl<'a, E: ExchangeMutableStreams<'a>> WebSocketStream<'a, E> {
    pub async fn subscribe(&self, subscription: &Subscription) -> Result<()> {
        self.exchange
            .subscribe(&self.inner_stream, subscription)
            .await
    }
}

impl<'a, E: ExchangeStreams<'a>> Stream for WebSocketStream<'a, E> {
    type Item = OpenLimitsWebsocketMessage;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(self.inner_stream.as_mut()).poll_next(cx)
    }
}
