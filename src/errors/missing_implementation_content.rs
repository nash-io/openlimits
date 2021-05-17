use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use thiserror::Error;

#[derive(Serialize, Deserialize, Debug, Error)]
pub struct MissingImplementationContent {
    pub message: String,
}

impl fmt::Display for MissingImplementationContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error message: {}", self.message)
    }
}