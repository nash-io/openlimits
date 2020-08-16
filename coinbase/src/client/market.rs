use crate::Coinbase;

use async_trait::async_trait;
use serde::Deserialize;
use shared::exchange_info::{get_pair, ExchangeInfoRetrieval, TradePair, TradePairHandle};
use shared::Result;
use std::fmt::Debug;

use crate::model::{
    Book, BookLevel, Candle, CandleRequestParams, Paginator, Product, Ticker, Trade,
};

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

    pub async fn trades(&self, pair: &str, paginator: Option<&Paginator>) -> Result<Vec<Trade>> {
        let endpoint = format!("/products/{}/trades", pair);
        self.transport.get(&endpoint, paginator).await
    }

    pub async fn ticker(&self, pair: &str) -> Result<Ticker> {
        let endpoint = format!("/products/{}/ticker", pair);
        self.transport.get::<_, ()>(&endpoint, None).await
    }

    pub async fn candles(
        &self,
        pair: &str,
        params: Option<&CandleRequestParams>,
    ) -> Result<Vec<Candle>> {
        let endpoint = format!("/products/{}/candles", pair);
        self.transport.get(&endpoint, params).await
    }

    pub async fn pair(&self, name: &str, refresh: bool) -> Result<Option<TradePairHandle>> {
        get_pair(name, &self.exchange_info, self, refresh).await
    }
}

#[async_trait]
impl ExchangeInfoRetrieval for Coinbase {
    async fn retrieve_pairs(&self) -> Result<Vec<(String, TradePair)>> {
        self.products().await.map(|v| {
            v.into_iter()
                .map(|product| {
                    (
                        product.id,
                        TradePair {
                            base: product.base_currency,
                            quote: product.quote_currency,
                            base_increment: product.base_increment,
                            quote_increment: product.quote_increment,
                        },
                    )
                })
                .collect()
        })
    }
}
