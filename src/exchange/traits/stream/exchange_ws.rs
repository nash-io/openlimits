use std::convert::TryInto;
use std::slice;
use async_trait::async_trait;
use futures::channel::mpsc::channel;
use futures::stream::BoxStream;
use futures::StreamExt;
use std::fmt::Debug;
use crate::errors::OpenLimitsError;
use crate::model::websocket::WebSocketResponse;
use crate::model::websocket::Subscription;
use crate::shared::Result;
use super::Subscriptions;
use super::CallbackHandle;


#[async_trait]
pub trait ExchangeWs: Send + Sync + Sized {
    type InitParams: Clone + Send + Sync + 'static;
    type Subscription: From<Subscription> + Send + Sync + Sized + Clone;
    type Response: TryInto<WebSocketResponse<Self::Response>, Error = OpenLimitsError>
        + Send
        + Sync
        + Clone
        + Sized
        + Debug
        + 'static;

    async fn new(params: Self::InitParams) -> Result<Self>;

    async fn disconnect(&self);

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
        let mut stream = self.create_stream_specific(s.into()).await?;

        let (mut tx, rx) = channel(1);

        tokio::spawn(async move {
            while let Some(Ok(message)) = stream.next().await {
                let message = message.try_into();
                callback(&message);
                tx.try_send(message).ok();
            }
            callback(&Err(OpenLimitsError::SocketError()));
        });

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