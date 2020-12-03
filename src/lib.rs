// REVIEW: I removed some of the deny's because they were slowing down the development and I made
// them `warn` again. Since most of them are `warn` by default, I simply removed them from the list.
// I kept the `unstable_features` as `deny` because it's critical.
#![deny(unstable_features)]
#![warn(unused_import_braces)]

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

pub mod any_exchange;
pub mod binance;
pub mod coinbase;
pub mod errors;
pub mod exchange;
pub mod exchange_info;
pub mod exchange_ws;
pub mod model;
pub mod nash;
pub mod shared;
