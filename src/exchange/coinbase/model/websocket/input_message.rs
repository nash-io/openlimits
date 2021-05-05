use serde::Deserialize;
use super::Level2SnapshotRecord;
use super::Level2UpdateRecord;
use super::Match;
use super::Activate;
use super::Done;
use super::Open;
use super::Received;
use super::Change;
use super::Channel;
use super::Ticker;

/// This enum represents the types of input messages
#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InputMessage {
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