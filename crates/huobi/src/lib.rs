pub mod prelude;

pub use exchange::*;

use libflate::gzip::Decoder;
use std::io::Read;

use async_trait::async_trait;
use nash_ws::{WebSocket, Message};

use uuid::Uuid;

pub mod message;

use message::HuobiResponse;
use exchange::message::subscription::Publication;

pub struct Huobi {
    sender: nash_ws::WebSocketSender,
    requests: Arc<Mutex<HashMap<Uuid, async_channel::Sender<HuobiResponse>>>>,
    subscriptions: Arc<Mutex<HashMap<String, async_channel::Sender<Publication>>>>
}

impl Huobi {
    async fn register_request(&mut self) -> (Uuid, async_channel::Receiver<HuobiResponse>) {
        let id = Uuid::new_v4();
        let (sender, receiver) = async_channel::bounded(1);
        self.requests.lock().await.insert(id, sender);
        (id, receiver)
    }

    async fn register_subscription(&mut self, subscription: String) -> async_channel::Receiver<Publication> {
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
            let subscriptions: Arc<Mutex<HashMap<String, async_channel::Sender<Publication>>>> = Arc::new(Mutex::new(HashMap::new()));
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
                                    if sender.send(Publication::Tick).await.is_err() {
                                        subscriptions.remove(&update.ch);
                                    }
                                },
                                HuobiMessage::Response(response) => {
                                    if let Some((_key, sender)) = requests.lock().await.remove_entry(&response.id) {
                                        sender.send(response).await.expect("Couldn't send.");
                                    }
                                }
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

use serde::{Serialize,Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use futures::lock::Mutex;
use crate::message::HuobiMessage;

#[derive(Serialize,Deserialize)]
struct HuobiSubscription {
    id: Uuid,
    sub: String
}