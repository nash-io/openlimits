use super::CoinbaseCredentials;

/// This struct represents the coinbase parameters
#[derive(Default, Clone, Debug)]
pub struct CoinbaseParameters {
    pub sandbox: bool,
    pub credentials: Option<CoinbaseCredentials>,
}

impl CoinbaseParameters {
    pub fn sandbox() -> Self {
        Self {
            sandbox: true,
            ..Default::default()
        }
    }

    pub fn prod() -> Self {
        Self {
            sandbox: false,
            ..Default::default()
        }
    }
}