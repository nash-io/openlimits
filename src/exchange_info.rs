use crate::shared::Result;
use async_trait::async_trait;
use rust_decimal::Decimal;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub async fn get_pair<'a>(
    name: &str,
    exchange_info: &'a ExchangeInfo,
    retrieval: &dyn ExchangeInfoRetrieval,
    refresh: bool,
) -> Result<Option<MarketPairHandle>> {
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
pub struct MarketPairHandle {
    pub inner: Arc<RwLock<MarketPair>>,
}

impl<'a> MarketPairHandle {
    fn new(inner: Arc<RwLock<MarketPair>>) -> Self {
        Self { inner }
    }

    pub fn read(&'a self) -> MarketPair {
        self.inner.read().unwrap().clone()
    }
}

impl<'a> serde::Serialize for MarketPairHandle {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        return serializer.collect_str(&self.inner.read().unwrap().symbol);
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

    pub fn get_pair(&self, name: &str) -> Option<MarketPairHandle> {
        let market_map = self.pairs.read().unwrap();
        let market_pair = market_map.get(name);
        market_pair.map(|inner| MarketPairHandle::new(inner.clone()))
    }

    pub async fn refresh(&self, retrieval: &dyn ExchangeInfoRetrieval) -> Result<()> {
        match retrieval.retrieve_pairs().await {
            Ok(pairs) => {
                if let Ok(mut writable_pairs) = self.pairs.write() {
                    for (id, pair) in pairs {
                        let entry = writable_pairs
                            .entry(id)
                            .or_insert_with(|| Arc::new(RwLock::new(pair.clone())));
                        if let Ok(mut entry) = entry.write() {
                            *entry = pair;
                        }
                    }
                }
            }
            Err(err) => return Err(err),
        }

        Ok(())
    }
}
