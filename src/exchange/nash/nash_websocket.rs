use std::{pin::Pin, task::Context, task::Poll};
use async_trait::async_trait;
use futures::stream::{BoxStream, SelectAll, Stream, StreamExt};
pub use nash_native_client::{Client, Environment};
use nash_protocol::protocol::ResponseOrError;
use crate::errors::OpenLimitsError;
use crate::exchange::traits::stream::{ExchangeWs, Subscriptions};
use super::NashParameters;
use super::utils::*;
use super::shared::Result;
use nash_protocol::protocol::subscriptions::{SubscriptionRequest, SubscriptionResponse};

/// This struct represents a websocket connection
pub struct NashWebsocket {
    pub client: Client,
}

impl Stream for NashWebsocket {
    type Item = std::result::Result<
        ResponseOrError<nash_protocol::protocol::subscriptions::SubscriptionResponse>,
        nash_protocol::errors::ProtocolError,
    >;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.client.poll_next_unpin(cx)
    }
}

#[async_trait]
impl ExchangeWs for NashWebsocket {
    type InitParams = NashParameters;
    type Subscription = SubscriptionRequest;
    type Response = SubscriptionResponse;

    async fn new(params: Self::InitParams) -> Result<Self> {
        Ok(Self {
            client: client_from_params_failable(params).await?,
        })
    }

    async fn disconnect(&self) {
        self.client.disconnect().await;
    }

    async fn create_stream_specific(
        &self,
        subscriptions: Subscriptions<Self::Subscription>,
    ) -> Result<BoxStream<'static, Result<Self::Response>>> {
        let mut streams = SelectAll::new();

        for subscription in subscriptions.into_iter() {
            let stream = self.client.subscribe_protocol(subscription).await?;
            streams.push(tokio_stream::wrappers::UnboundedReceiverStream::new(stream));
        }

        let s = streams.map(|message| match message {
            Ok(msg) => match msg {
                ResponseOrError::Response(resp) => Ok(resp.data),
                ResponseOrError::Error(resp) => {
                    let f = resp
                        .errors
                        .iter()
                        .map(|f| f.message.clone())
                        .collect::<Vec<String>>()
                        .join("\n");
                    Err(OpenLimitsError::NotParsableResponse(f))
                }
            },
            Err(_) => Err(OpenLimitsError::SocketError()),
        });

        Ok(s.boxed())
    }
}