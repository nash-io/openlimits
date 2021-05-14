use derive_more::Constructor;
use serde::Deserialize;
use serde::Serialize;

/// This struct represents the cancellation of all orders
#[derive(Serialize, Deserialize, Clone, Constructor, Debug, PartialEq)]
pub struct CancelAllOrdersRequest {
    pub market_pair: Option<String>,
}