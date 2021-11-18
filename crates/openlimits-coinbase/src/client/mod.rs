//! This module is used to make calls to api and connect to the websockets

mod account;
mod market;
mod base_client;
pub mod websocket;

pub use base_client::BaseClient;
pub use super::shared;
pub (crate) use super::transport::Transport;

