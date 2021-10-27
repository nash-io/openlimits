use derive_more::Constructor;
use serde::Deserialize;
use serde::Serialize;

/// This struct represents a transaction
#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct Transaction<T> {
    pub id: T,
    pub market_pair: String,
    pub client_order_id: Option<String>,
    pub created_at: u64,
}
