use serde::Deserialize;

/// This enum represents the stop type
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum StopType {
    Entry,
    Exit,
}