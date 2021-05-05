use serde::Deserialize;
use serde::Serialize;

/// This enum contains the channel types
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