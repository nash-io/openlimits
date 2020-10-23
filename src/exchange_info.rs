use crate::{errors::OpenLimitError, shared::Result};
use async_trait::async_trait;
use rust_decimal::Decimal;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub fn get_pair<'a>(name: &str, exchange_info: &'a ExchangeInfo) -> Result<MarketPairHandle> {
    exchange_info.get_pair(name)
}

#[async_trait]
pub trait ExchangeInfoRetrieval: Sync {
    async fn retrieve_pairs(&self) -> Result<Vec<MarketPair>>;
    async fn refresh_market_info(&self) -> Result<Vec<MarketPairHandle>>;
}

#[derive(Debug, Clone)]
pub struct MarketPair {
    pub base: String,
    pub quote: String,
    pub symbol: String,
    pub base_increment: Decimal,
    pub quote_increment: Decimal,
}

#[derive(Debug)]
pub struct MarketPairHandle {
    pub inner: Arc<RwLock<MarketPair>>,
}

impl<'a> MarketPairHandle {
    fn new(inner: Arc<RwLock<MarketPair>>) -> Self {
        Self { inner }
    }

    pub fn read(&'a self) -> Result<MarketPair> {
        self.inner
            .read()
            .map(|guard| guard.clone())
            .map_err(|_| OpenLimitError::PoisonError())
    }
}

impl<'a> serde::Serialize for MarketPairHandle {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.inner.read().unwrap().symbol)
    }
}

#[derive(Clone)]
pub struct ExchangeInfo {
    pairs: Arc<RwLock<HashMap<String, Arc<RwLock<MarketPair>>>>>,
}

impl ExchangeInfo {
    pub fn new() -> Self {
        Self {
            pairs: Arc::new(RwLock::new(HashMap::default())),
        }
    }

    pub fn get_pair(&self, name: &str) -> Result<MarketPairHandle> {
        let market_map = self.pairs.read().unwrap();
        let market_pair = market_map.get(name);
        market_pair.map_or(Err(OpenLimitError::SymbolNotFound()), |inner| {
            Ok(MarketPairHandle::new(inner.clone()))
        })
    }

    pub fn list_pairs(&self) -> Vec<MarketPairHandle> {
        let market_map = self.pairs.read().unwrap();
        market_map
            .iter()
            .map(|(_symbol, market)| MarketPairHandle::new(market.clone()))
            .collect()
    }

    pub async fn refresh(
        &self,
        retrieval: &dyn ExchangeInfoRetrieval,
    ) -> Result<Vec<MarketPairHandle>> {
        let pairs = retrieval.retrieve_pairs().await?;

        if let Ok(mut writable_pairs) = self.pairs.write() {
            for pair in pairs {
                let entry = writable_pairs
                    .entry(pair.symbol.clone())
                    .or_insert_with(|| Arc::new(RwLock::new(pair.clone())));
                if let Ok(mut entry) = entry.write() {
                    *entry = pair;
                }
            }
        }

        Ok(self.list_pairs())
    }
}

impl Default for ExchangeInfo {
    fn default() -> Self {
        ExchangeInfo::new()
    }
}
