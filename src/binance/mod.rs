pub mod client;
pub mod model;
mod transport;

use transport::Transport;

#[derive(Clone)]
pub struct Binance {
    pub transport: Transport,
}

impl Binance {
    pub fn new(sandbox: bool) -> Self {
        Binance {
            transport: Transport::new(sandbox).unwrap(),
        }
    }

    pub fn with_credential(api_key: &str, api_secret: &str, sandbox: bool) -> Self {
        Binance {
            transport: Transport::with_credential(api_key, api_secret, sandbox).unwrap(),
        }
    }
}
