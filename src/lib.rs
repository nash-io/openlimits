#![deny(
    unstable_features,
    unused_must_use,
    unused_mut,
    unused_imports,
    unused_import_braces
)]

extern crate chrono;
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
extern crate derive_more;

pub mod binance;
pub mod coinbase;
pub mod openlimits;
pub mod shared;
