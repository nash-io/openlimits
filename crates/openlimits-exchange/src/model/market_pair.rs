use serde::{Serialize, Deserialize};
pub use crate::model::currency::Currency;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub struct MarketPair(pub Currency, pub Currency);