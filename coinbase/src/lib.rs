#![deny(
    unstable_features,
    unused_must_use,
    unused_mut,
    unused_imports,
    unused_import_braces
)]

extern crate chrono;
extern crate hex;
extern crate hmac;
extern crate log;
extern crate serde;
extern crate serde_json;
extern crate sha2;
extern crate shared;
extern crate sugar;
extern crate tokio;
extern crate tokio_tungstenite;
extern crate tungstenite;
extern crate url;

pub mod client;
pub mod model;
mod transport;

use shared::exchange_info::ExchangeInfo;
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
