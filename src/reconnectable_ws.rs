use crate::model::websocket::{WebSocketResponse, Subscription};
use std::sync::Arc;
use crate::exchange_ws::{ExchangeWs, OpenLimitsWs, Subscriptions, CallbackHandle};
use tokio::sync::Mutex;
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use crate::shared::Result;
use tokio::time::Duration;
use std::thread::sleep;
use futures::stream::BoxStream;
use crate::errors::OpenLimitsError;

pub type SubscriptionCallback<Response> = Arc<dyn Fn(&Result<WebSocketResponse<Response>>) + Sync + Send + 'static>;

pub struct ReconnectableWebsocket<E: ExchangeWs> {
    websocket: Arc<Mutex<OpenLimitsWs<E>>>,
    tx: UnboundedSender<()>,
    subscriptions: Arc<Mutex<Vec<(Subscription, SubscriptionCallback<E::Response>)>>>
}

impl<E: ExchangeWs + 'static> ReconnectableWebsocket<E> {
    pub async fn instantiate(params: E::InitParams, reattempt_interval: Duration) -> Result<Self> {
        let websocket = E::new(params.clone()).await?;
        let websocket = OpenLimitsWs { websocket };
        let websocket = Arc::new(Mutex::new(websocket));
        let subscriptions: Arc<Mutex<Vec<(Subscription, SubscriptionCallback<E::Response>)>>> = Arc::new(Mutex::new(Default::default()));
        let (tx, mut rx) = unbounded_channel();
        {
            let websocket = Arc::downgrade(&websocket);
            let subscriptions = Arc::downgrade(&subscriptions);
            tokio::spawn(async move {
                while let Some(_) = rx.recv().await {
                    'reconnection: loop {
                        if let (Some(websocket), Some(subscriptions)) = (websocket.upgrade(), subscriptions.upgrade()) {
                            if let Ok(new_websocket) = E::new(params.clone()).await {
                                let new_websocket = OpenLimitsWs { websocket: new_websocket };
                                let mut websocket = websocket.lock().await;
                                *websocket = new_websocket;

                                let subscriptions = {
                                    subscriptions.lock().await.clone()
                                };
                                let subscriptions = subscriptions.iter().map(|(subscription, callback)| {
                                    let callback = callback.clone();
                                    websocket.subscribe(subscription.clone(), move |message| {
                                        callback(message)
                                    })
                                });
                                if futures_util::future::join_all(subscriptions).await.iter().all(|subscription| subscription.is_ok()) {
                                    break 'reconnection;
                                }
                            }
                            println!("Couldn't connect. Trying again.");
                            sleep(reattempt_interval);
                        }
                    }
                }
            });
        }
        Ok(Self { websocket, tx, subscriptions })
    }

    pub async fn create_stream_specific(
        &self,
        subscriptions: Subscriptions<E::Subscription>,
    ) -> Result<BoxStream<'static, Result<E::Response>>> {
        self.websocket.lock().await.create_stream_specific(subscriptions).await
    }

    pub async fn subscribe<
        F: Fn(&Result<WebSocketResponse<E::Response>>) + Sync + Send + Clone + 'static,
    >(
        &self,
        subscription: Subscription,
        callback: F,
    ) -> Result<CallbackHandle> {
        let tx = self.tx.clone();
        self.subscriptions.lock().await.push((subscription.clone(), Arc::new(callback.clone())));
        self.websocket.lock().await.subscribe(subscription, move |message| {
            if let Err(OpenLimitsError::SocketError()) = message.as_ref() {
                tx.send(()).ok();
            }
            callback(message);
        }).await
    }

    pub async fn create_stream<S: Into<E::Subscription> + Clone + Send + Sync>(
        &self,
        subscriptions: &[S],
    ) -> Result<BoxStream<'static, Result<WebSocketResponse<E::Response>>>> {
        self.websocket.lock().await.create_stream(subscriptions).await
    }
}