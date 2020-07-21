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

#[derive(Error, Debug)]
pub enum OpenLimitError {
    #[error("")]
    BinanceError(#[from] BinanceContentError),
    #[error("")]
    CoinbaseError(#[from] CoinbaseContentError),
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
    GetTimestampFailed(),
    #[error("")]
    ReqError(#[from] reqwest::Error),
    #[error("")]
    InvalidHeaderError(#[from] reqwest::header::InvalidHeaderValue),
    #[error("")]
    InvalidPayloadSignature(#[from] serde_urlencoded::ser::Error),
    #[error("")]
    IoError(#[from] std::io::Error),
    #[error("")]
    JsonError(#[from] serde_json::Error),
    #[error("")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("")]
    UrlParserError(#[from] url::ParseError),
    #[error("")]
    Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("")]
    TimestampError(#[from] std::time::SystemTimeError),
    #[error("")]
    UnkownResponse(String),
    #[error("")]
    NotParsableResponse(String),
    #[error("")]
    MissingParameter(String),
}
