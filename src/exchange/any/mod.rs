//! In some contexts, such as bindings in other languages (e.g., Python via pyo3), it is not possible to use trait
//! constraints on generics. This module provides an enum wrapper type for all openlimits exchanges that code can
//! use to operate over any openlimits-supported exchange without generics

mod any_exchange;
mod any_ws_exchange;
mod init_any_exchange;

pub use any_exchange::AnyExchange;
pub use any_ws_exchange::AnyWsExchange;
pub use init_any_exchange::InitAnyExchange;