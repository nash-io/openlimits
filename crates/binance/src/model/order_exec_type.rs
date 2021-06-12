use serde::Deserialize;
use serde::Serialize;

/// This enum represents the type of executed order
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderExecType {
    New,
}