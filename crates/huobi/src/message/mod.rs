use uuid::Uuid;
use rust_decimal::Decimal;

mod requester;
mod subscriber;

pub use requester::*;
pub use subscriber::*;

use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
#[serde(untagged)]
pub enum HuobiMessage {
    Response(HuobiResponse),
    ChannelUpdate(ChannelUpdate),
    Ping(Ping)
}

#[derive(Debug,Serialize,Deserialize)]
pub struct HuobiResponse {
    pub id: Uuid,
    pub status: String,
    pub subbed: String,
    pub ts: u64
}

#[derive(Debug,Serialize,Deserialize)]
pub struct ChannelUpdate {
    pub ch: String,
    pub ts: u64,
    pub tick: Tick
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Tick {
    pub id: u64,
    pub open: Decimal,
    pub close: Decimal,
    pub low: Decimal,
    pub high: Decimal,
    pub amount: Decimal,
    pub vol: Decimal,
    pub count: u64
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Ping {
    pub ping: u64
}
