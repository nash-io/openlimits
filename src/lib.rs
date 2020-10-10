// REVIEW: I removed some of the deny's because they were slowing down the development and I made
// them `warn` again. Since most of them are `warn` by default, I simply removed them from the list.
// I kept the `unstable_features` as `deny` because it's critical.
#![deny(unstable_features)]
#![warn(unused_import_braces)]

#[cfg(not(target_arch = "wasm32"))]
pub mod binance;
#[cfg(not(target_arch = "wasm32"))]
pub mod coinbase;
pub mod errors;
pub mod exchange;
pub mod exchange_info;
#[cfg(not(target_arch = "wasm32"))]
pub mod exchange_ws;
pub mod model;
#[cfg(not(target_arch = "wasm32"))]
pub mod nash;
pub mod shared;

#[cfg(not(target_arch = "wasm32"))]
pub mod any_exchange;
