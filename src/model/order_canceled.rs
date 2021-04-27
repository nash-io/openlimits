use derive_more::Constructor;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct OrderCanceled {
    pub id: String,
}