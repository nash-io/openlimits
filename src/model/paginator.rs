use serde::Deserialize;
use serde::Serialize;

/// This struct represents a paginator
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Paginator {
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub limit: Option<u64>,
    pub before: Option<String>,
    pub after: Option<String>,
}