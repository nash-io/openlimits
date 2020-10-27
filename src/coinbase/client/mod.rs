mod account;
mod market;
pub mod websocket;

#[derive(Clone)]
pub struct BaseClient {
    pub transport: super::transport::Transport
}