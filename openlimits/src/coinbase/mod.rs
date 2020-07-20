use async_trait::async_trait;
use derive_more::{Deref, DerefMut};
use shared::Result;

use crate::exchange::Exchange;
use crate::model::{Asks, Bids, OrderBookRequest, OrderBookResponse};

#[derive(Deref, DerefMut)]
pub struct Coinbase(coinbase::Coinbase);

impl Coinbase {
    pub fn new(sandbox: bool) -> Self {
        Coinbase(coinbase::Coinbase::new(sandbox))
    }

    pub fn with_credential(
        api_key: &str,
        api_secret: &str,
        passphrase: &str,
        sandbox: bool,
    ) -> Self {
        Coinbase(coinbase::Coinbase::with_credential(
            api_key, api_secret, passphrase, sandbox,
        ))
    }
}

#[async_trait]
impl Exchange for Coinbase {
    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse> {
        self.book::<coinbase::model::BookRecordL2>("BTC-USD")
            .await
            .map(Into::into)
    }
}

impl From<coinbase::model::Book<coinbase::model::BookRecordL2>> for OrderBookResponse {
    fn from(book: coinbase::model::Book<coinbase::model::BookRecordL2>) -> Self {
        Self {
            last_update_id: None,
            bids: book.bids.into_iter().map(Into::into).collect(),
            asks: book.asks.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<coinbase::model::BookRecordL2> for Bids {
    fn from(bids: coinbase::model::BookRecordL2) -> Self {
        Self {
            price: bids.price,
            qty: bids.size,
        }
    }
}

impl From<coinbase::model::BookRecordL2> for Asks {
    fn from(bids: coinbase::model::BookRecordL2) -> Self {
        Self {
            price: bids.price,
            qty: bids.size,
        }
    }
}
