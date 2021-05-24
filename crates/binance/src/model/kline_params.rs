use serde::Deserialize;
use serde::Serialize;
use super::Paginator;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KlineParams {
    pub symbol: String,
    pub interval: String,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paginator: Option<Paginator>,
}