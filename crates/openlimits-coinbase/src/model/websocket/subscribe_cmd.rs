use serde::Deserialize;
use serde::Serialize;

/// This enum represents a subscribe command
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SubscribeCmd {
    Subscribe,
}