use serde::Deserialize;
use serde::Serialize;

/// This enum represents a time interval
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Interval {
    Second,
    Minute,
    Day,
}