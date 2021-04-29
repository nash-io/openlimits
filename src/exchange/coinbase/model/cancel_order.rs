use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CancelOrder {
    pub product_id: Option<String>,
}