use serde::Deserialize;
use serde::Serialize;

/// This struct represents a paginator
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Paginator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<u64>,
}