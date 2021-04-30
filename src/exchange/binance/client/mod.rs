mod account;
mod general;
mod market;
mod userstream;
pub mod websocket;
pub use super::shared;

#[derive(Clone)]
pub struct BaseClient {
    pub transport: super::transport::Transport,
}
