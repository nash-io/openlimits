use serde::Deserialize;

/// This enum represents the stop type
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum StopType {
    Entry,
    Exit,
}