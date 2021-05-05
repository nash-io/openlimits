use serde::Deserialize;
use serde::Serialize;

/// This struct represents the server time
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub server_time: u64,
}