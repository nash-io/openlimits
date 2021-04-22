#![deny(unstable_features)]
#![allow(clippy::too_many_arguments)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unsafe_code)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]

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
pub mod reconnectable_ws;
