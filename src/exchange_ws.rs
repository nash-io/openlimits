use crate::{
    exchange::ExchangeSpec,
    model::websocket::{OpenLimitsWebsocketMessage, Subscription},
};
use async_trait::async_trait;
use derive_more::Constructor;
use futures::stream::Stream;

use crate::shared::Result;
use std::{marker::PhantomData, pin::Pin, task::Context, task::Poll};

#[derive(Constructor)]
pub struct OpenLimitsWs<E: ExchangeWs, S: ExchangeSpec> {
    pub websocket: E,
    phantom: PhantomData<S>,
}

impl<E: ExchangeWs, S: ExchangeSpec> OpenLimitsWs<E, S> {
    pub async fn subscribe(&mut self, subscription: Subscription) -> Result<()> {
        self.websocket.subscribe(subscription).await
    }
}

#[async_trait]
pub trait ExchangeWs: Stream + Unpin {
    async fn subscribe(&mut self, subscription: Subscription) -> Result<()>;
    fn parse_message<S: ExchangeSpec>(
        &self,
        message: Self::Item,
    ) -> Result<OpenLimitsWebsocketMessage<S>>;
}

impl<E: ExchangeWs, S: ExchangeSpec> Stream for OpenLimitsWs<E, S> {
    type Item = Result<OpenLimitsWebsocketMessage<S>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Poll::Ready(Some(message)) = Pin::new(&mut self.websocket).poll_next(cx) {
            let m = self.websocket.parse_message(message);

            return Poll::Ready(Some(m));
        }

        Poll::Pending
    }
}
