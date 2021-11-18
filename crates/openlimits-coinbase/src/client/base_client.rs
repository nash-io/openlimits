use super::Transport;

/// The coinbase client
#[derive(Clone, Debug)]
pub struct BaseClient {
    pub transport: Transport,
}