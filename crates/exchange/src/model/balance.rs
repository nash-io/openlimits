use derive_more::Constructor;
use rust_decimal::prelude::Decimal;
use serde::Deserialize;
use serde::Serialize;

/// This struct represents the account balance
#[derive(Serialize, Deserialize, Clone, Constructor, Debug, PartialEq)]
pub struct Balance {
    pub asset: String,
    pub total: Decimal,
    pub free: Decimal,
}