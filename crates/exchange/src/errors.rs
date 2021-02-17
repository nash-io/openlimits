use serde::{Deserialize, Serialize};
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

#[derive(Error, Debug)]
pub enum Error {
    #[error("")]
    NoMarketPair,
    #[error("")]
    MissingImplementation(#[from] MissingImplementationContent),
    #[error("")]
    AssetNotFound(),
    #[error("")]
    NoApiKeySet(),
    #[error("")]
    InternalServerError(),
    #[error("")]
    ServiceUnavailable(),
    #[error("")]
    Unauthorized(),
    #[error("")]
    SymbolNotFound(),
    #[error("")]
    SocketError(),
    #[error("")]
    WebSocketMessageNotSupported(),
    #[error("")]
    GetTimestampFailed(),
    #[error(transparent)]
    InvalidPayloadSignature(#[from] serde_urlencoded::ser::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("")]
    PoisonError(),
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error(transparent)]
    UrlParserError(#[from] url::ParseError),
    #[error(transparent)]
    TimestampError(#[from] std::time::SystemTimeError),
    #[error("")]
    UnkownResponse(String),
    #[error("")]
    NotParsableResponse(String),
    #[error("")]
    MissingParameter(String),
    #[error("")]
    InvalidParameter(String),
}
