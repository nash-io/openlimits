use crate::Coinbase;

use async_trait::async_trait;
use serde::Deserialize;
use shared::exchange_info::{get_pair, ExchangeInfoRetrieval, MarketPair, MarketPairHandle};
use shared::Result;
use std::fmt::Debug;

use crate::model::{
    Book, BookLevel, Candle, CandleRequestParams, Paginator, Product, Ticker, Trade,
};

impl Coinbase {
    pub async fn products(&self) -> Result<Vec<Product>> {
        self.transport.get::<_, ()>("/products", None).await
    }

    pub async fn product(&self, market_pair: &MarketPairHandle) -> Result<Product> {
        let market_pair = market_pair.inner.read().unwrap();

        let endpoint = format!("/products/{}", market_pair.symbol);
        self.transport.get::<_, ()>(&endpoint, None).await
    }

    pub async fn book<T>(&self, market_pair: &MarketPairHandle) -> Result<Book<T>>
    where
        T: BookLevel + Debug + 'static,
        T: for<'de> Deserialize<'de>,
    {
        let market_pair = market_pair.inner.read().unwrap();
        let endpoint = format!("/products/{}/book?level={}", market_pair.symbol, T::level());
        self.transport.get::<_, ()>(&endpoint, None).await
    }

    pub async fn trades(
        &self,
        market_pair: &MarketPairHandle,
        paginator: Option<&Paginator>,
    ) -> Result<Vec<Trade>> {
        let market_pair = market_pair.inner.read().unwrap();

        let endpoint = format!("/products/{}/trades", market_pair.symbol);
        self.transport.get(&endpoint, paginator).await
    }

    pub async fn ticker(&self, market_pair: &MarketPairHandle) -> Result<Ticker> {
        let market_pair = market_pair.inner.read().unwrap();

        let endpoint = format!("/products/{}/ticker", market_pair.symbol);
        self.transport.get::<_, ()>(&endpoint, None).await
    }

    pub async fn candles(
        &self,
        market_pair: &MarketPairHandle,
        params: Option<&CandleRequestParams>,
    ) -> Result<Vec<Candle>> {
        let market_pair = market_pair.inner.read().unwrap();

        let endpoint = format!("/products/{}/candles", market_pair.symbol);
        self.transport.get(&endpoint, params).await
    }

    pub async fn pair(&self, name: &str, refresh: bool) -> Result<Option<MarketPairHandle>> {
        get_pair(name, &self.exchange_info, self, refresh).await
    }
}

#[async_trait]
impl ExchangeInfoRetrieval for Coinbase {
    async fn retrieve_pairs(&self) -> Result<Vec<(String, MarketPair)>> {
        self.products().await.map(|v| {
            v.into_iter()
                .map(|product| {
                    (
                        product.id.clone(),
                        MarketPair {
                            symbol: product.id,
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
