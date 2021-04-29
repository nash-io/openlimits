use std::sync::Arc;
use std::sync::RwLock;
use crate::errors::OpenLimitsError;
use crate::shared::Result;
use super::MarketPair;


#[derive(Debug)]
pub struct MarketPairHandle {
    pub inner: Arc<RwLock<MarketPair>>,
}

impl<'a> MarketPairHandle {
    pub fn new(inner: Arc<RwLock<MarketPair>>) -> Self {
        Self { inner }
    }

    pub fn read(&'a self) -> Result<MarketPair> {
        self.inner
            .read()
            .map(|guard| guard.clone())
            .map_err(|_| OpenLimitsError::PoisonError())
    }
}

impl<'a> serde::Serialize for MarketPairHandle {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(&self.inner.read().expect("Couldn't read pairs.").symbol)
    }
}