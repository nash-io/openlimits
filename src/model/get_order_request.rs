use derive_more::Constructor;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct GetOrderRequest {
    pub id: String,
    pub market_pair: Option<String>,
}