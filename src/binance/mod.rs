pub mod client;
pub mod model;
mod transport;

use crate::shared::exchange_info::ExchangeInfo;
use transport::Transport;

#[derive(Clone)]
pub struct Binance {
    exchange_info: ExchangeInfo,
    transport: Transport,
}

impl Binance {
    pub fn new(sandbox: bool) -> Self {
        Binance {
            exchange_info: ExchangeInfo::new(),
            transport: Transport::new(sandbox).unwrap(),
        }
    }

    pub fn with_credential(api_key: &str, api_secret: &str, sandbox: bool) -> Self {
        Binance {
            exchange_info: ExchangeInfo::new(),
            transport: Transport::with_credential(api_key, api_secret, sandbox).unwrap(),
        }
    }
}
