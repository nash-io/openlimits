use std::{any::Any, convert::TryInto};

use crate::{
    errors::OpenLimitError,
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

    async fn new(params: Self::InitParams) -> Self;

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

        let (tx, rx) = channel(1);

        tokio::spawn(fut.map(Ok).skip_while(|_| future::ready(true)).forward(tx));

        Ok(CallbackHandle { rx: Box::new(rx) })
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

#[derive(Debug)]
pub struct CallbackHandle {
    rx: Box<dyn Any + Send>,
}
