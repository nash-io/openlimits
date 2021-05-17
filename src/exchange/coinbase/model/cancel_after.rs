use serde::Deserialize;
use serde::Serialize;

/// This enum represents a cancel after order
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CancelAfter {
    Min,
    Hour,
    Day,
}