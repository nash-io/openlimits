use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;
use std::fmt;

/// This struct represents the coinbase content errors
#[derive(Serialize, Deserialize, Debug, Error)]
pub struct CoinbaseContentError {
    pub message: String,
}

impl fmt::Display for CoinbaseContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error message: {}", self.message)
    }
}