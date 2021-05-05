use serde::Deserialize;
use super::Level2SnapshotRecord;
use super::Level2UpdateRecord;

/// This enum represents the level 2
#[derive(Deserialize, Debug, Clone)]
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