use serde::Deserialize;
use serde::Serialize;
use super::Paginator;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AllOrderReq {
    pub symbol: String,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paginator: Option<Paginator>,
}