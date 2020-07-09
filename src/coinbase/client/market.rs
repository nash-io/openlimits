use crate::coinbase::Coinbase;
use crate::Result;
use serde::Deserialize;
use std::fmt::Debug;

use crate::coinbase::model::{Book, BookLevel, Product, Trade};

impl Coinbase {
    // That is probably the same as get_exchange_info for binance.const
    pub async fn products(&self) -> Result<Vec<Product>> {
        self.transport.get::<_, ()>("/products", None).await
    }

    pub async fn product(&self, pair: &str) -> Result<Product> {
        let endpoint = format!("/products/{}", pair);
        self.transport.get::<_, ()>(&endpoint, None).await
    }

    pub async fn book<T>(&self, pair: &str) -> Result<Book<T>>
    where
        T: BookLevel + Debug + 'static,
        T: for<'de> Deserialize<'de>,
    {
        let endpoint = format!("/products/{}/book?level={}", pair, T::level());
        self.transport.get::<_, ()>(&endpoint, None).await
    }

    pub async fn trades(&self, pair: &str) -> Result<Vec<Trade>> {
        let endpoint = format!("/products/{}/trades", pair);
        self.transport.get::<_, ()>(&endpoint, None).await
    }
}
