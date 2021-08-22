use derive_more::Constructor;
use serde::Deserialize;
use serde::Serialize;

/// This struct represents a canceled order
#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct OrderCanceled {
    pub id: String,
}

// TODO: Verify this.
impl From<String> for OrderCanceled {
    fn from(id: String) -> Self {
        Self { id }
    }
}
