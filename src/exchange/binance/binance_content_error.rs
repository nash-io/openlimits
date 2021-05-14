use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;


/// This struct represents a binance content error
#[derive(Serialize, Deserialize, Debug, Error)]
pub struct BinanceContentError {
    pub code: i16,
    pub msg: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

impl fmt::Display for BinanceContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error code: {} msg: {}", self.code, self.msg)
    }
}