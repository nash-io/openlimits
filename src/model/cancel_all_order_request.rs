use derive_more::Constructor;
use serde::Deserialize;
use serde::Serialize;


#[derive(Serialize, Deserialize, Clone, Constructor, Debug, PartialEq)]
pub struct CancelAllOrdersRequest {
    pub market_pair: Option<String>,
}