use super::shared::Result;
use super::ExchangeInfo;
use super::MarketPairHandle;

pub fn get_pair<'a>(name: &str, exchange_info: &'a ExchangeInfo) -> Result<MarketPairHandle> {
    exchange_info.get_pair(name)
}