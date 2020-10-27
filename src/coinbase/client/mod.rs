mod account;
mod market;
pub mod websocket;

#[derive(Clone)]
pub struct Client {
    pub transport: super::transport::Transport
}