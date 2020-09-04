extern crate openlimits;

mod any_exchange;
#[cfg(not(target_arch="wasm32"))]
mod apis;
#[cfg(not(target_arch="wasm32"))]
mod binance;
#[cfg(not(target_arch="wasm32"))]
mod coinbase;
mod nash;
