#![deny(
    unstable_features,
    unused_must_use,
    unused_mut,
    unused_imports,
    unused_import_braces
)]

extern crate chrono;
extern crate derive_more;
extern crate hex;
extern crate hmac;
extern crate log;
extern crate serde;
extern crate serde_json;
extern crate sha2;
extern crate sugar;
extern crate tokio;
extern crate tokio_tungstenite;
extern crate tungstenite;
extern crate url;

pub mod binance;
pub mod coinbase;
pub mod errors;
pub mod exchange;
pub mod exchange_info;
pub mod exchange_ws;
pub mod model;
pub mod nash;
pub mod shared;
pub mod any_exchange;
