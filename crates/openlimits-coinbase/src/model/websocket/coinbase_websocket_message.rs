use serde::Deserialize;
use serde::Deserializer;
use super::Channel;
use super::Ticker;
use super::Level2;
use super::Match;
use super::InputMessage;
use super::Full;

/// This enum represents a coinbase websocket message
#[derive(Debug, Clone, PartialEq)]
pub enum CoinbaseWebsocketMessage {
    Subscriptions {
        channels: Vec<Channel>,
    },
    Heartbeat {
        sequence: usize,
        last_trade_id: usize,
        product_id: String,
        time: String,
    },
    Ticker(Ticker),
    Level2(Level2),
    Match(Match),
    Full(Full),
    Error {
        message: String,
    },
}

impl<'de> Deserialize<'de> for CoinbaseWebsocketMessage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|input_msg: InputMessage| input_msg.into())
    }
}

impl From<InputMessage> for CoinbaseWebsocketMessage {
    fn from(msg: InputMessage) -> Self {
        match msg {
            InputMessage::Subscriptions { channels } => {
                CoinbaseWebsocketMessage::Subscriptions { channels }
            }
            InputMessage::Heartbeat {
                sequence,
                last_trade_id,
                product_id,
                time,
            } => CoinbaseWebsocketMessage::Heartbeat {
                sequence,
                last_trade_id,
                product_id,
                time,
            },
            InputMessage::Ticker(ticker) => CoinbaseWebsocketMessage::Ticker(ticker),
            InputMessage::Snapshot {
                product_id,
                bids,
                asks,
            } => CoinbaseWebsocketMessage::Level2(Level2::Snapshot {
                product_id,
                bids,
                asks,
            }),
            InputMessage::L2update {
                product_id,
                changes,
            } => CoinbaseWebsocketMessage::Level2(Level2::L2update {
                product_id,
                changes,
            }),
            InputMessage::LastMatch(_match) => CoinbaseWebsocketMessage::Match(_match),
            InputMessage::Received(_match) => {
                CoinbaseWebsocketMessage::Full(Full::Received(_match))
            }
            InputMessage::Open(open) => CoinbaseWebsocketMessage::Full(Full::Open(open)),
            InputMessage::Done(done) => CoinbaseWebsocketMessage::Full(Full::Done(done)),
            InputMessage::Match(_match) => CoinbaseWebsocketMessage::Full(Full::Match(_match)),
            InputMessage::Change(change) => CoinbaseWebsocketMessage::Full(Full::Change(change)),
            InputMessage::Activate(activate) => {
                CoinbaseWebsocketMessage::Full(Full::Activate(activate))
            }
            InputMessage::Error { message } => CoinbaseWebsocketMessage::Error { message },
        }
    }
}