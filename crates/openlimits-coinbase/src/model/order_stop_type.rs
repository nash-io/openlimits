use serde::Deserialize;
use serde::Serialize;

/// This struct represents the type stop of an order
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum OrderStopType {
    Loss,
    Entry,
}