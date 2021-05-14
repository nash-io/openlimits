//! In some contexts, such as bindings in other languages (e.g., Python via pyo3), it is not possible to use trait
//! constraints on generics. This module provides an enum wrapper type for all openlimits exchanges that code can
//! use to operate over any openlimits-supported exchange without generics
//! 
//! # Example
//! ```
//! use openlimits::exchange::any::AnyExchange;
//! use openlimits::exchange::any::InitAnyExchange;
//! use openlimits::exchange::binance::BinanceParameters;
//! use openlimits::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Binance, Coinbase and Nash availables
//!     let binance = AnyExchange::new(InitAnyExchange::Binance(BinanceParameters::prod()))
//!                     .await
//!                     .expect("Couldn't create binance client");

//!     let order_book = binance.order_book(&OrderBookRequest {market_pair: "BTCEUR".to_string()})
//!                     .await
//!                     .expect("Couldn't get order book");

//!     println!("{:?}", order_book);
//! }
//! ```

mod any_exchange;
mod any_ws_exchange;
mod init_any_exchange;

pub use any_exchange::AnyExchange;
pub use any_ws_exchange::AnyWsExchange;
pub use init_any_exchange::InitAnyExchange;
pub use super::shared;