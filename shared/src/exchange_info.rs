use super::Result;
use async_trait::async_trait;
use rust_decimal::Decimal;

use std::{
    collections::HashMap,
    sync::{Arc, RwLockReadGuard, RwLock},
};

pub async fn get_pair<'a>(
    name: &str,
    exchange_info: &'a mut ExchangeInfo,
    retrieval: &dyn ExchangeInfoRetrieval,
    refresh: bool,
) -> Result<Option<MarketPairHandle<'a>>> {
    if refresh {
        if let Err(err) = exchange_info.refresh(retrieval).await {
            return Err(err);
        }
    }

    Ok(exchange_info.get_pair(name))
}

#[async_trait]
pub trait ExchangeInfoRetrieval: Sync {
    async fn retrieve_pairs(&self) -> Result<Vec<(String, MarketPair)>>;
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
pub struct MarketPairHandle<'a> {
    pub inner: RwLockReadGuard<'a, MarketPair>,
}

impl<'a> MarketPairHandle<'a> {
    fn new(inner: RwLockReadGuard<'a, MarketPair>) -> Self {
        Self { inner }
    }
}

impl<'a> serde::Serialize for MarketPairHandle<'a> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        return serializer.collect_str(&self.inner.symbol );
    }
}

#[derive(Clone)]
pub struct ExchangeInfo {
    pairs: HashMap<String, Arc<RwLock<MarketPair>>>,
}

impl ExchangeInfo {
    pub fn new() -> Self {
        Self {
            pairs: HashMap::default(),
        }
    }

    pub fn get_pair(&self, name: &str) -> Option<MarketPairHandle> {
        self.pairs
            .get(name)
            .map(|pair| pair.read())
            .map(|inner| MarketPairHandle::new(inner.unwrap()))
    }

    pub async fn refresh(&mut self, retrieval: &dyn ExchangeInfoRetrieval) -> Result<()> {
        match retrieval.retrieve_pairs().await {
            Ok(pairs) => {
                for (id, pair) in pairs {
                    let entry = self.pairs
                        .entry(id)
                        .or_insert_with(|| Arc::new(RwLock::new(pair.clone())));
                    if let Ok(mut entry) = entry.write() {
                        *entry = pair;
                    }
                }
            }
            Err(err) => return Err(err),
        }

        Ok(())
    }
}
