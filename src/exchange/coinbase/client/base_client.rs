use super::Transport;

/// The coinbase client
#[derive(Clone)]
pub struct BaseClient {
    pub transport: Transport,
}