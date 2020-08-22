pub mod client;
pub mod model;
mod transport;

use crate::shared::exchange_info::ExchangeInfo;
use transport::Transport;

#[derive(Clone)]
pub struct Coinbase {
    exchange_info: ExchangeInfo,
    transport: Transport,
}

impl Coinbase {
    pub fn new(sandbox: bool) -> Self {
        Coinbase {
            exchange_info: ExchangeInfo::new(),
            transport: Transport::new(sandbox).unwrap(),
        }
    }

    pub fn with_credential(
        api_key: &str,
        api_secret: &str,
        passphrase: &str,
        sandbox: bool,
    ) -> Self {
        Coinbase {
            exchange_info: ExchangeInfo::new(),
            transport: Transport::with_credential(api_key, api_secret, passphrase, sandbox)
                .unwrap(),
        }
    }
}
