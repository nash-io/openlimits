use serde::Deserialize;
use serde::Serialize;
use super::ChannelType;

/// This enum represents a channel
#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Channel {
    Name(ChannelType),
    WithProduct {
        name: ChannelType,
        product_ids: Vec<String>,
    },
}