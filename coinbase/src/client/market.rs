use crate::Coinbase;

use serde::Deserialize;
use std::fmt::Debug;

use shared::Result;

use crate::model::{Book, BookLevel, Candle, Product, Ticker, Trade};

impl Coinbase {
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

    pub async fn ticker(&self, pair: &str) -> Result<Ticker> {
        let endpoint = format!("/products/{}/ticker", pair);
        self.transport.get::<_, ()>(&endpoint, None).await
    }

    pub async fn candles(&self, pair: &str) -> Result<Vec<Candle>> {
        let endpoint = format!("/products/{}/candles", pair);
        self.transport.get::<_, ()>(&endpoint, None).await
    }
}
