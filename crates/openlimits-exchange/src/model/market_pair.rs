use serde::{Serialize, Deserialize};
pub use crate::model::currency::Currency;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub struct MarketPair(pub Currency, pub Currency);

impl MarketPair {
    pub fn inverse(&self) -> MarketPair {
        MarketPair(self.1.clone(), self.0.clone())
    }
}