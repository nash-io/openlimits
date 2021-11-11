use super::CoinbaseCredentials;
use openlimits_exchange::exchange::Environment;

/// This struct represents the coinbase parameters
#[derive(Default, Clone, Debug)]
pub struct CoinbaseParameters {
    pub environment: Environment,
    pub credentials: Option<CoinbaseCredentials>,
}

impl CoinbaseParameters {
    pub fn sandbox() -> Self {
        Self {
            environment: Environment::Sandbox,
            ..Default::default()
        }
    }

    pub fn production() -> Self {
        Self {
            environment: Environment::Production,
            ..Default::default()
        }
    }
}