mod account;
mod general;
mod market;
mod userstream;
pub mod websocket;

#[derive(Clone)]
pub struct BaseClient {
    pub transport: super::transport::Transport
}