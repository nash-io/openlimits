use super::BinanceCredentials;

/// This struct represents the type of environment that will be used and receives a boolean and the credentials as parameters.
#[derive(Default, Clone, Debug)]
pub struct BinanceParameters {
    pub sandbox: bool,
    pub credentials: Option<BinanceCredentials>,
}

impl BinanceParameters {
    /// Sandbox environment
    pub fn sandbox() -> Self {
        Self {
            sandbox: true,
            ..Default::default()
        }
    }

    /// Production environment
    pub fn prod() -> Self {
        Self {
            sandbox: false,
            ..Default::default()
        }
    }
}