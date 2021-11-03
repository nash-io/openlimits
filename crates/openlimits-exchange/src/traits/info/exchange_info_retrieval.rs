use async_trait::async_trait;
use super::shared::Result;
use super::MarketPairHandle;
use super::MarketPairInfo;
use crate::model::market_pair::MarketPair;

/// This struct represents the information retrieval
#[async_trait]
pub trait ExchangeInfoRetrieval: Sync {
    async fn get_pair(&self, market_pair: &MarketPair) -> Result<MarketPairHandle>;
    async fn retrieve_pairs(&self) -> Result<Vec<MarketPairInfo>>;
    async fn refresh_market_info(&self) -> Result<Vec<MarketPairHandle>>;
}