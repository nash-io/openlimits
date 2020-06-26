#![deny(
    unstable_features,
    unused_must_use,
    unused_mut,
    unused_imports,
    unused_import_braces
)]

extern crate hex;
extern crate hmac;
extern crate serde;
extern crate sha2;
extern crate serde_json;
extern crate log;
extern crate chrono;
extern crate sugar;
extern crate tokio;
extern crate tokio_tungstenite;
extern crate tungstenite;
extern crate url;

pub mod errors;
pub mod client;
pub mod exchange;
pub mod binance;

use errors::OpenLimitError;
pub(crate) type Result<T> = std::result::Result<T, OpenLimitError>;
