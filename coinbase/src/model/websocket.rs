extern crate serde;

use rust_decimal::prelude::Decimal;
use serde::{Deserialize, Deserializer, Serialize};
use shared::{string_to_decimal, string_to_opt_decimal};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Subscription {
    pub product_ids: Vec<String>,
    pub channels: Vec<ChannelType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
    pub signature: String,
    pub key: String,
    pub passphrase: String,
    pub timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Subscribe {
    #[serde(rename = "type")]
    pub _type: SubscribeCmd,
    pub product_ids: Vec<String>,
    pub channels: Vec<Channel>,
    #[serde(flatten)]
    pub auth: Option<Auth>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SubscribeCmd {
    Subscribe,
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Channel {
    Name(ChannelType),
    WithProduct {
        name: ChannelType,
        product_ids: Vec<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ChannelType {
    Heartbeat,
    Ticker,
    Level2,
    Matches,
    Full,
    User,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub(crate) enum InputMessage {
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
    Snapshot {
        product_id: String,
        bids: Vec<Level2SnapshotRecord>,
        asks: Vec<Level2SnapshotRecord>,
    },
    L2update {
        product_id: String,
        changes: Vec<Level2UpdateRecord>,
    },
    LastMatch(Match),
    Received(Received),
    Open(Open),
    Done(Done),
    Match(Match),
    Activate(Activate),
    Change(Change),
    Error {
        message: String,
    },
}

#[derive(Debug)]
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

#[derive(Deserialize, Debug)]
pub enum Level2 {
    Snapshot {
        product_id: String,
        bids: Vec<Level2SnapshotRecord>,
        asks: Vec<Level2SnapshotRecord>,
    },
    L2update {
        product_id: String,
        changes: Vec<Level2UpdateRecord>,
    },
}

#[derive(Deserialize, Debug)]
pub struct Level2SnapshotRecord {
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub size: Decimal,
}

#[derive(Deserialize, Debug)]
pub struct Level2UpdateRecord {
    pub side: super::OrderSide,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub size: Decimal,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum Ticker {
    Full {
        trade_id: usize,
        sequence: usize,
        time: String,
        product_id: String,
        #[serde(with = "string_to_decimal")]
        price: Decimal,
        side: super::OrderSide,
        #[serde(with = "string_to_decimal")]
        last_size: Decimal,
        #[serde(with = "string_to_opt_decimal")]
        best_bid: Option<Decimal>,
        #[serde(with = "string_to_opt_decimal")]
        best_ask: Option<Decimal>,
    },
    Empty {
        sequence: usize,
        product_id: String,
        #[serde(with = "string_to_opt_decimal")]
        price: Option<Decimal>,
    },
}

impl Ticker {
    pub fn price(&self) -> Decimal {
        match self {
            Ticker::Full { price, .. } => *price,
            Ticker::Empty { price, .. } => price.unwrap(),
        }
    }

    pub fn time(&self) -> Option<&String> {
        match self {
            Ticker::Full { time, .. } => Some(time),
            Ticker::Empty { .. } => None,
        }
    }

    pub fn sequence(&self) -> &usize {
        match self {
            Ticker::Full { sequence, .. } => sequence,
            Ticker::Empty { sequence, .. } => sequence,
        }
    }

    pub fn bid(&self) -> Option<Decimal> {
        match self {
            Ticker::Full { best_bid, .. } => Some(best_bid.unwrap()),
            Ticker::Empty { .. } => None,
        }
    }

    pub fn ask(&self) -> Option<Decimal> {
        match self {
            Ticker::Full { best_ask, .. } => Some(best_ask.unwrap()),
            Ticker::Empty { .. } => None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum Full {
    Received(Received),
    Open(Open),
    Done(Done),
    Match(Match),
    Change(Change),
    Activate(Activate),
}

impl Full {
    pub fn price(&self) -> Option<&Decimal> {
        match self {
            Full::Received(Received::Limit { price, .. }) => Some(price),
            Full::Received(Received::Market { .. }) => None,
            Full::Open(Open { price, .. }) => Some(price),
            Full::Done(Done::Limit { price, .. }) => Some(price),
            Full::Done(Done::Market { .. }) => None,
            Full::Match(Match { price, .. }) => Some(price),
            Full::Change(Change { price, .. }) => price.as_ref(),
            Full::Activate(Activate { .. }) => None,
        }
    }

    pub fn time(&self) -> Option<&String> {
        match self {
            Full::Received(Received::Limit { time, .. }) => Some(time),
            Full::Received(Received::Market { time, .. }) => Some(time),
            Full::Open(Open { time, .. }) => Some(time),
            Full::Done(Done::Limit { time, .. }) => Some(time),
            Full::Done(Done::Market { time, .. }) => Some(time),
            Full::Match(Match { time, .. }) => Some(time),
            Full::Change(Change { time, .. }) => Some(time),
            Full::Activate(Activate { .. }) => None,
        }
    }

    pub fn sequence(&self) -> Option<&usize> {
        match self {
            Full::Received(Received::Limit { sequence, .. }) => Some(sequence),
            Full::Received(Received::Market { sequence, .. }) => Some(sequence),
            Full::Open(Open { sequence, .. }) => Some(sequence),
            Full::Done(Done::Limit { sequence, .. }) => sequence.as_ref(),
            Full::Done(Done::Market { sequence, .. }) => Some(sequence),
            Full::Match(Match { sequence, .. }) => Some(sequence),
            Full::Change(Change { sequence, .. }) => Some(sequence),
            Full::Activate(Activate { .. }) => None,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "order_type")]
#[serde(rename_all = "camelCase")]
pub enum Received {
    Limit {
        time: String,
        product_id: String,
        sequence: usize,
        order_id: String,
        client_oid: Option<String>,
        #[serde(with = "string_to_decimal")]
        size: Decimal,
        #[serde(with = "string_to_decimal")]
        price: Decimal,
        side: super::OrderSide,
        user_id: Option<String>,
        #[serde(default)]
        profile_id: Option<String>,
    },
    Market {
        time: String,
        product_id: String,
        sequence: usize,
        order_id: String,
        client_oid: Option<String>,
        #[serde(default)]
        #[serde(with = "string_to_opt_decimal")]
        funds: Option<Decimal>,
        side: super::OrderSide,
    },
}

#[derive(Deserialize, Debug)]
pub struct Open {
    pub time: String,
    pub product_id: String,
    pub sequence: usize,
    pub order_id: String,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub remaining_size: Decimal,
    pub side: super::OrderSide,
    pub user_id: Option<String>,
    #[serde(default)]
    pub profile_id: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Done {
    Limit {
        time: String,
        product_id: String,
        sequence: Option<usize>,
        #[serde(with = "string_to_decimal")]
        price: Decimal,
        order_id: String,
        reason: Reason,
        side: super::OrderSide,
        #[serde(with = "string_to_decimal")]
        remaining_size: Decimal,
        user_id: Option<String>,
        #[serde(default)]
        profile_id: Option<String>,
    },
    Market {
        time: String,
        product_id: String,
        sequence: usize,
        order_id: String,
        reason: Reason,
        side: super::OrderSide,
    },
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Reason {
    Filled,
    Canceled,
}

#[derive(Deserialize, Debug)]
pub struct Match {
    pub trade_id: usize,
    pub sequence: usize,
    pub maker_order_id: String,
    pub taker_order_id: String,
    pub time: String,
    pub product_id: String,
    #[serde(with = "string_to_decimal")]
    pub size: Decimal,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    pub side: super::OrderSide,
    pub taker_user_id: Option<String>,
    pub taker_profile_id: Option<String>,
    pub maker_user_id: Option<String>,
    pub maker_profile_id: Option<String>,
    pub user_id: Option<String>,
    #[serde(default)]
    pub profile_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Change {
    pub time: String,
    pub sequence: usize,
    pub order_id: String,
    pub product_id: String,
    #[serde(with = "string_to_decimal")]
    pub new_size: Decimal,
    #[serde(with = "string_to_decimal")]
    pub old_size: Decimal,
    #[serde(default)]
    #[serde(with = "string_to_opt_decimal")]
    pub new_funds: Option<Decimal>,
    #[serde(default)]
    #[serde(with = "string_to_opt_decimal")]
    pub old_funds: Option<Decimal>,
    #[serde(default)]
    #[serde(with = "string_to_opt_decimal")]
    pub price: Option<Decimal>,
    pub side: super::OrderSide,
    pub user_id: Option<String>,
    #[serde(default)]
    pub profile_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Activate {
    pub product_id: String,
    #[serde(with = "string_to_decimal")]
    pub timestamp: Decimal,
    pub order_id: String,
    pub stop_type: StopType,
    #[serde(with = "string_to_decimal")]
    pub size: Decimal,
    #[serde(with = "string_to_decimal")]
    pub funds: Decimal,
    #[serde(with = "string_to_decimal")]
    pub taker_fee_rate: Decimal,
    pub private: bool,
    pub user_id: Option<String>,
    #[serde(default)]
    pub profile_id: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum StopType {
    Entry,
    Exit,
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

impl<'de> Deserialize<'de> for CoinbaseWebsocketMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|input_msg: InputMessage| input_msg.into())
    }
}
