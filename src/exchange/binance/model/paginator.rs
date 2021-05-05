use serde::Deserialize;
use serde::Serialize;

/// This struct represents a paginator
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Paginator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<u64>,
}