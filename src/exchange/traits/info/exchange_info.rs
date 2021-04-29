use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use crate::errors::OpenLimitsError;
use crate::shared::Result;
use super::ExchangeInfoRetrieval;
use super::MarketPairHandle;
use super::MarketPair;

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
        let market_map = self.pairs.read().expect("Couldn't read pairs.");
        let market_pair = market_map.get(name);
        market_pair.map_or(Err(OpenLimitsError::SymbolNotFound()), |inner| {
            Ok(MarketPairHandle::new(inner.clone()))
        })
    }

    pub fn list_pairs(&self) -> Vec<MarketPairHandle> {
        let market_map = self.pairs.read().expect("Couldn't read pairs.");
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