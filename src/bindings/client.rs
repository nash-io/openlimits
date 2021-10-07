use crate::exchange::coinbase::{CoinbaseParameters, Coinbase};
use crate::prelude::*;

mod ask_bid;
pub use ask_bid::FFIAskBid;

pub mod coinbase;

mod runtime {
    use ligen_macro::inner_ligen;
    inner_ligen!(ignore);
    mod runtime {
        lazy_static::lazy_static! {
            pub static ref RUNTIME: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
        }
    }
    pub use runtime::RUNTIME;
}

use runtime::RUNTIME;
use crate::model::AskBid;
use rust_decimal::Decimal;

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
