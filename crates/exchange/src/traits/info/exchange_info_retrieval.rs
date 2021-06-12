use async_trait::async_trait;
use super::shared::Result;
use super::MarketPairHandle;
use super::MarketPair;

/// This struct represents the information retrieval
#[async_trait]
pub trait ExchangeInfoRetrieval: Sync {
    async fn get_pair(&self, name: &str) -> Result<MarketPairHandle>;
    async fn retrieve_pairs(&self) -> Result<Vec<MarketPair>>;
    async fn refresh_market_info(&self) -> Result<Vec<MarketPairHandle>>;
}