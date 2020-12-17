use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

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

#[derive(Serialize, Deserialize, Debug, Error)]
pub struct CoinbaseContentError {
    pub message: String,
}

impl fmt::Display for CoinbaseContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error message: {}", self.message)
    }
}

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
pub enum OpenLimitError {
    #[error("")]
    NoMarketPair,
    #[error(transparent)]
    BinanceError(#[from] BinanceContentError),
    #[error(transparent)]
    CoinbaseError(#[from] CoinbaseContentError),
    #[error(transparent)]
    NashProtocolError(#[from] nash_protocol::errors::ProtocolError),
    #[error(transparent)]
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
    ReqError(#[from] reqwest::Error),
    #[error(transparent)]
    InvalidHeaderError(#[from] reqwest::header::InvalidHeaderValue),
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
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),
    #[error(transparent)]
    TimestampError(#[from] std::time::SystemTimeError),
    #[error("")]
    UnkownResponse(String),
    #[error("")]
    NotParsableResponse(String),
    #[error("")]
    MissingParameter(String),
}
