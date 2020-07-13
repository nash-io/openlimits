mod transport;

pub mod client;
pub mod model;

use transport::Transport;

#[derive(Clone)]
pub struct Coinbase {
    pub transport: Transport,
}

impl Coinbase {
    pub fn new(sandbox: bool) -> Self {
        Coinbase {
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
            transport: Transport::with_credential(api_key, api_secret, passphrase, sandbox)
                .unwrap(),
        }
    }
}
