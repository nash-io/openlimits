use super::CoinbaseCredentials;

#[derive(Default, Clone)]
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