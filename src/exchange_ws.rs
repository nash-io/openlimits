use async_trait::async_trait;
use derive_more::Constructor;
use futures::stream::Stream;
use crate::model::websocket::{Subscription, OpenLimitsWebsocketMessage};

use crate::shared::Result;
use std::task::Poll;

#[derive(Constructor)]
pub struct OpenLimitsWs<E: ExchangeWs> {
    pub websocket: E,
}

#[async_trait]
pub trait ExchangeWs {
    type WebSocketMessageType;
    fn subscribe(&self, subscription: Subscription) -> Result<()>;
    fn parse_message(&self, message: Self::WebSocketMessageType) -> Result<OpenLimitsWebsocketMessage>;
}

impl Stream for OpenLimitsWs<ExchangeWs>  {
    type Item = Result<OpenLimitsWebsocketMessage>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {

        if let Poll::Ready(Some(message)) = self.websocket.poll_next(cx) {
            let m = self.parse_message( message?);

            return Poll::Ready(Some(m));
        }
        
        std::task::Poll::Pending
    }
}
