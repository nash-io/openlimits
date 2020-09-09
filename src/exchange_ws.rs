use crate::model::websocket::{OpenLimitsWebsocketMessage, Subscription};
use async_trait::async_trait;
use derive_more::Constructor;
use futures::stream::Stream;

use crate::shared::Result;
use std::{pin::Pin, task::Context, task::Poll};

#[derive(Constructor)]
pub struct OpenLimitsWs<E: ExchangeWs> {
    pub websocket: E,
}

#[async_trait]
pub trait ExchangeWs: Stream + Unpin {
    type WebSocketMessageType;
    fn subscribe(&self, subscription: Subscription) -> Result<()>;
    fn parse_message(
        &self,
        message: Self::WebSocketMessageType,
    ) -> Result<OpenLimitsWebsocketMessage>;
}

impl<E: ExchangeWs> Stream for OpenLimitsWs<E> {
    type Item = Result<OpenLimitsWebsocketMessage>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let Poll::Ready(Some(message)) = Pin::new(&mut self.websocket).poll_next(cx) {
            let m = self.websocket.parse_message(message);

            return Poll::Ready(Some(m));
        }

        Poll::Pending
    }
}
