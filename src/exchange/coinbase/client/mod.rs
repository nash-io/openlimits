mod account;
mod market;
pub mod websocket;
pub use super::shared;

#[derive(Clone)]
pub struct BaseClient {
    pub transport: super::transport::Transport,
}
