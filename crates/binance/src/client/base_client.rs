use super::Transport;

/// The binance client
#[derive(Clone)]
pub struct BaseClient {
    pub transport: Transport,
}