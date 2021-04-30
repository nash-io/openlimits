use async_trait::async_trait;
use super::shared::Result;
use super::info::ExchangeInfoRetrieval;
use super::ExchangeAccount;
use super::ExchangeMarketData;

#[async_trait]
pub trait Exchange: ExchangeInfoRetrieval + ExchangeAccount + ExchangeMarketData + Sized {
    type InitParams;
    type InnerClient;
    async fn new(params: Self::InitParams) -> Result<Self>;
    fn inner_client(&self) -> Option<&Self::InnerClient>;
}