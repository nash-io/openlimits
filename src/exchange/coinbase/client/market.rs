use serde::Deserialize;
use std::fmt::Debug;
use crate::{
    exchange::coinbase::model::{
        Book, BookLevel, Candle, CandleRequestParams, Paginator, Product, Ticker, Trade,
    },
};
use super::shared::Result;
use super::BaseClient;
use crate::exchange::coinbase::model::MarketPair;

impl BaseClient {
    pub async fn products(&self) -> Result<Vec<Product>> {
        self.transport.get::<_, ()>("/products", None).await
    }

    pub async fn product(&self, pair: &str) -> Result<Product> {
        let endpoint = format!("/products/{}", pair);
        self.transport.get::<_, ()>(&endpoint, None).await
    }

    pub async fn book<T, P>(&self, pair: P) -> Result<Book<T>>
    where
        T: BookLevel + Debug + 'static,
        T: for<'de> Deserialize<'de>,
        P: Into<MarketPair>
    {
        let endpoint = format!("/products/{}/book?level={}", pair.into().0, T::level());
        self.transport.get::<_, ()>(&endpoint, None).await
    }

    pub async fn trades<P: Into<MarketPair>>(&self, pair: P, paginator: Option<&Paginator>) -> Result<Vec<Trade>> {
        let endpoint = format!("/products/{}/trades", pair.into().0);
        self.transport.get(&endpoint, paginator).await
    }

    pub async fn ticker<P: Into<MarketPair>>(&self, pair: P) -> Result<Ticker> {
        let endpoint = format!("/products/{}/ticker", pair.into().0);
        self.transport.get::<_, ()>(&endpoint, None).await
    }

    pub async fn candles<P: Into<MarketPair>>(
        &self,
        pair: P,
        params: Option<&CandleRequestParams>,
    ) -> Result<Vec<Candle>> {
        let endpoint = format!("/products/{}/candles", pair.into().0);
        self.transport.get(&endpoint, params).await
    }
}
