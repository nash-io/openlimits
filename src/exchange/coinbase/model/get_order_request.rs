use serde::Deserialize;
use serde::Serialize;
use super::Paginator;

/// This structs a request of an oder
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GetOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paginator: Option<Paginator>,
}