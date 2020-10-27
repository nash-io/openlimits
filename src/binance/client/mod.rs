mod account;
mod general;
mod market;
mod userstream;
pub mod websocket;

#[derive(Clone)]
pub struct Client {
    transport: super::transport::Transport
}