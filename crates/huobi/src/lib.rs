pub mod prelude;

mod exchange;
pub use exchange::*;

use libflate::gzip::Decoder;
use std::io::Read;

use futures::stream::Stream;

mod request_response;
use request_response::*;
pub use request_response::Subscription;

use futures_util::StreamExt;

use async_trait::async_trait;
use nash_ws::{WebSocket, Message};

use rust_decimal::Decimal;

use uuid::Uuid;

pub struct Huobi {
    sender: nash_ws::WebSocketSender,
    requests: Arc<Mutex<HashMap<Uuid, async_channel::Sender<HuobiResponse>>>>,
    subscriptions: Arc<Mutex<HashMap<String, async_channel::Sender<SubscriptionUpdate>>>>
}

impl Huobi {
    async fn new_request(&mut self) -> (Uuid, async_channel::Receiver<HuobiResponse>) {
        let id = Uuid::new_v4();
        let (sender, receiver) = async_channel::bounded(1);
        self.requests.lock().await.insert(id, sender);
        (id, receiver)
    }

    async fn new_subscription(&mut self, subscription: String) -> async_channel::Receiver<SubscriptionUpdate> {
        let (sender, receiver) = async_channel::bounded(1);
        self.subscriptions.lock().await.insert(subscription, sender);
        receiver
    }
}

pub struct HuobiParameters {
    pub environment: Environment
}

impl HuobiParameters {
    pub fn new(environment: Environment) -> Self {
        Self { environment }
    }
}

#[derive(Debug,Serialize,Deserialize)]
#[serde(untagged)]
pub enum HuobiMessage {
    Response(HuobiResponse),
    ChannelUpdate(ChannelUpdate),
    Ping(Ping)
}

#[derive(Debug,Serialize,Deserialize)]
pub struct HuobiResponse {
    id: Uuid,
    status: String,
    subbed: String,
    ts: u64
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ChannelUpdate {
    ch: String,
    ts: u64,
    tick: Tick
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Tick {
    id: u64,
    open: Decimal,
    close: Decimal,
    low: Decimal,
    high: Decimal,
    amount: Decimal,
    vol: Decimal,
    count: u64
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Ping {
    ping: u64
}

#[async_trait]
impl Exchange for Huobi {
    type InitializationParameters = HuobiParameters;
    fn endpoint_url(environment: Environment) -> &'static str {
        match environment {
            Environment::Production => "wss://api.huobi.pro/ws",
            Environment::Sandbox => unimplemented!("Huobi doesn't have a sandbox endpoint.")
        }
    }

    async fn new(parameters: Self::InitializationParameters) -> Result<Self> {
        let websocket = WebSocket::new(Self::endpoint_url(parameters.environment)).await;
        websocket.map(|websocket| {
            let requests: Arc<Mutex<HashMap<_, async_channel::Sender<HuobiResponse>>>> = Arc::new(Mutex::new(HashMap::new()));
            let subscriptions: Arc<Mutex<HashMap<String, async_channel::Sender<SubscriptionUpdate>>>> = Arc::new(Mutex::new(HashMap::new()));
            let (sender, mut receiver) = websocket;
            {
                let mut websocket = sender.clone();
                let subscriptions = subscriptions.clone();
                let requests = requests.clone();
                cross_async::spawn(async move {
                    while let Some(Ok(message)) = receiver.next().await {
                        if let Message::Binary(binary) = message {
                            let mut decoder = Decoder::new(&binary[..]).expect("Couldn't create Decoder.");
                            let mut str = String::new();
                            decoder.read_to_string(&mut str).expect("Couldn't read to String.");
                            let message = serde_json::from_str(&str).expect("Couldn't parse");
                            match message {
                                HuobiMessage::Ping(ping) => {
                                    websocket.send(&Message::Text(format!("{{\"pong\": {}}}", ping.ping))).await.ok();
                                },
                                HuobiMessage::ChannelUpdate(update) => {
                                    let mut subscriptions = subscriptions.lock().await;
                                    let sender = subscriptions.get(&update.ch).expect("Getting an unregistered subscription should never happend.");
                                    if sender.send(SubscriptionUpdate::Tick).await.is_err() {
                                        subscriptions.remove(&update.ch);
                                    }
                                },
                                HuobiMessage::Response(response) => {
                                    if let Some((_key, sender)) = requests.lock().await.remove_entry(&response.id) {
                                        sender.send(response).await.expect("Couldn't send.");
                                    }
                                }
                                _ => ()
                            }
                        }
                    }
                    subscriptions.lock().await.clear();
                });
            }
            Self { sender, requests, subscriptions }
        }).map_err(|error| format!("{:#?}", error))
    }
}

#[async_trait]
impl Requester for Huobi {
    async fn request(&mut self, request: &Request) -> Result<Response> {
        let (id, mut receiver) = self.new_request().await;
        let request_message = match request {
            Request::Subscription(subscription) => {
                let sub = format!("market.{}{}.kline.1min", "eth", "btc");
                let request = HuobiSubscription { id, sub };
                serde_json::to_string(&request)
                    .map_err(|error| {
                        format!("{:#?}", error)
                    })?
            }
        };
        self.sender.send(&Message::Text(request_message)).await.expect("Couldn't send message.");
        let response = receiver.next().await;
        response
            .filter(|response| response.status == "ok")
            .map(|response| {
                Response::Subscription(SubscriptionResponse{})
            })
            .ok_or_else(|| {
                // FIXME: Also implement timeout.
                "Failed to receive resposne".into()
            })
    }
}

use serde::{Serialize,Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use futures::lock::Mutex;

#[derive(Serialize,Deserialize)]
struct HuobiSubscription {
    id: Uuid,
    sub: String
}

use crate::exchange::SubscriptionUpdate;

#[async_trait]
impl Subscriber for Huobi {
    async fn subscribe(&mut self, subscription: Subscription) -> Result<SubscriptionStream> {
        let receiver = self.new_subscription("market.ethbtc.kline.1min".to_string()).await;
        let response = self.request(&Request::Subscription(subscription)).await?;
        Ok(Box::pin(receiver))
    }
}