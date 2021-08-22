use super::Transport;

/// The openlimits-binance client
#[derive(Clone)]
pub struct BaseClient {
    pub transport: Transport,
}