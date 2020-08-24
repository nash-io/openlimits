use crate::shared::Result;
use async_trait::async_trait;
use rust_decimal::Decimal;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub async fn get_pair(
    name: &str,
    exchange_info: &ExchangeInfo,
    retrieval: &dyn ExchangeInfoRetrieval,
    refresh: bool,
) -> Result<Option<TradePairHandle>> {
    if refresh {
        if let Err(err) = exchange_info.refresh(retrieval).await {
            return Err(err);
        }
    }

    Ok(exchange_info.get_pair(name))
}

#[async_trait]
pub trait ExchangeInfoRetrieval: Sync {
    async fn retrieve_pairs(&self) -> Result<Vec<(String, TradePair)>>;
}

#[derive(Debug, Clone)]
pub struct TradePair {
    pub base: String,
    pub quote: String,
    pub symbol: String,
    pub base_increment: Decimal,
    pub quote_increment: Decimal,
}

#[derive(Clone, Debug)]
pub struct TradePairHandle {
    inner: Arc<RwLock<TradePair>>,
}

impl TradePairHandle {
    fn new(inner: Arc<RwLock<TradePair>>) -> Self {
        Self { inner }
    }
}

#[derive(Clone)]
pub struct ExchangeInfo {
    pairs: Arc<RwLock<HashMap<String, Arc<RwLock<TradePair>>>>>,
}

impl ExchangeInfo {
    pub fn new() -> Self {
        Self {
            pairs: Arc::new(RwLock::new(HashMap::default())),
        }
    }

    pub fn get_pair(&self, name: &str) -> Option<TradePairHandle> {
        self.pairs
            .read()
            .unwrap()
            .get(name)
            .map(|pair| TradePairHandle::new(pair.clone()))
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
