pub use exchange::errors::MissingImplementationContent;
pub use anyhow::Result;
pub use exchange::errors::Error as OpenLimitsError;
//
// #[derive(Error, Debug)]
// pub enum OpenLimitsError {
//     #[error("")]
//     NoMarketPair,
//     #[error(transparent)]
//     BinanceError(#[from] BinanceContentError),
//     #[error(transparent)]
//     CoinbaseError(#[from] CoinbaseContentError),
//     #[error(transparent)]
//     NashProtocolError(#[from] nash_protocol::errors::ProtocolError),
//     #[error(transparent)]
//     MissingImplementation(#[from] MissingImplementationContent),
//     #[error("")]
//     AssetNotFound(),
//     #[error("")]
//     NoApiKeySet(),
//     #[error("")]
//     InternalServerError(),
//     #[error("")]
//     ServiceUnavailable(),
//     #[error("")]
//     Unauthorized(),
//     #[error("")]
//     SymbolNotFound(),
//     #[error("")]
//     SocketError(),
//     #[error("")]
//     WebSocketMessageNotSupported(),
//     #[error("")]
//     GetTimestampFailed(),
//     #[error(transparent)]
//     ReqError(#[from] reqwest::Error),
//     #[error(transparent)]
//     InvalidHeaderError(#[from] reqwest::header::InvalidHeaderValue),
//     #[error(transparent)]
//     InvalidPayloadSignature(#[from] serde_urlencoded::ser::Error),
//     #[error(transparent)]
//     IoError(#[from] std::io::Error),
//     #[error("")]
//     PoisonError(),
//     #[error(transparent)]
//     JsonError(#[from] serde_json::Error),
//     #[error(transparent)]
//     ParseFloatError(#[from] std::num::ParseFloatError),
//     #[error(transparent)]
//     UrlParserError(#[from] url::ParseError),
//     #[error(transparent)]
//     Tungstenite(#[from] tokio_tungstenite::tungstenite::Error),
//     #[error(transparent)]
//     TimestampError(#[from] std::time::SystemTimeError),
//     #[error("")]
//     UnkownResponse(String),
//     #[error("")]
//     NotParsableResponse(String),
//     #[error("")]
//     MissingParameter(String),
//     #[error("")]
//     InvalidParameter(String),
// }
