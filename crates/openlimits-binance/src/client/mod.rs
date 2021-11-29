//! This module is used to make calls to api and connect to the websockets
mod account;
mod general;
mod market;
mod userstream;
mod base_client;
pub mod stream;

pub use base_client::BaseClient;
pub (crate) use super::transport::Transport;
pub use super::shared;

