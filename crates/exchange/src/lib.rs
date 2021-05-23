mod exchange;
pub use crate::exchange::*;
pub mod market;
pub mod message;
pub mod errors;
pub mod prelude;

pub mod model;

pub type Error = String;
pub type Result<T> = std::result::Result<T, Error>;