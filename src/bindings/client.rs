use rust_decimal::Decimal;

use runtime::RUNTIME;

pub use crate::bindings::ask_bid::FFIAskBid;
use crate::exchange::coinbase::{Coinbase, CoinbaseParameters};
use crate::model::AskBid;
use crate::prelude::*;

pub mod coinbase;

mod runtime {
    use ligen_macro::inner_ligen;

    pub use runtime::RUNTIME;

    inner_ligen!(ignore);
    mod runtime {
        lazy_static::lazy_static! {
            pub static ref RUNTIME: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
        }
    }
}

#[repr(C)]
pub struct Client {
    client: *mut Coinbase
}

impl Client {
    pub fn coinbase(parameters: CoinbaseParameters) -> Self {
        let client = crate::OpenLimits::instantiate(parameters);
        let client = RUNTIME.block_on(client);
        let client = client.unwrap();
        let client = Box::into_raw(Box::new(client));
        Self { client }
    }

    pub fn sum(self, a: Vec<u64>) -> u64 {
        a.iter().sum()
    }

    pub fn mul(self, a: Vec<u64>, n: u64) -> Vec<u64> {
        a.iter().map(|x| x * n).collect()
    }

    pub fn order_book(self, market_pair: String) -> AskBid {
        unsafe {
            if let Some(client) = self.client.as_ref() {
                let response = RUNTIME.block_on(client.order_book(&OrderBookRequest { market_pair }));
                let response = response.unwrap();
                response.asks[0]
            } else {
                let price = Decimal::new(1, 1);
                let qty = Decimal::new(1, 1);
                AskBid { price, qty }
            }
        }
    }
}
