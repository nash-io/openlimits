use crate::{
    binance::{
        model::{ExchangeInformation, ServerTime, SymbolFilter},
        Binance,
    },
    exchange_info::{ExchangeInfoRetrieval, MarketPair, MarketPairHandle},
    shared::Result,
};
use async_trait::async_trait;
use serde_json::Value;

impl Binance {
    // Test connectivity
    pub async fn ping(&self) -> Result<String> {
        self.transport
            .get::<_, ()>("/api/v1/ping", None)
            .await
            .map(|_: Value| "pong".into())
    }

    // Check server time
    pub async fn get_server_time(&self) -> Result<ServerTime> {
        self.transport.get::<_, ()>("/api/v1/time", None).await
    }

    pub async fn get_exchange_info(&self) -> Result<ExchangeInformation> {
        self.transport
            .get::<_, ()>("/api/v1/exchangeInfo", None)
            .await
    }

    pub fn get_pair(&self, name: &str) -> Result<MarketPairHandle> {
        self.exchange_info.get_pair(name)
    }
}

#[async_trait]
impl ExchangeInfoRetrieval for Binance {
    async fn retrieve_pairs(&self) -> Result<Vec<MarketPair>> {
        self.get_exchange_info().await.map(|v| {
            v.symbols
                .into_iter()
                .map(|symbol| {
                    let lot_size = symbol
                        .filters
                        .iter()
                        .find_map(|f| match f {
                            SymbolFilter::LotSize {
                                max_qty: _,
                                min_qty: _,
                                step_size,
                            } => Some(step_size),
                            _ => None,
                        })
                        .unwrap();

                    let tick_size = symbol
                        .filters
                        .iter()
                        .find_map(|f| match f {
                            SymbolFilter::PriceFilter {
                                min_price: _,
                                max_price: _,
                                tick_size,
                            } => Some(tick_size),
                            _ => None,
                        })
                        .unwrap();

                    MarketPair {
                        base: symbol.base_asset,
                        quote: symbol.quote_asset,
                        symbol: symbol.symbol,
                        base_increment: *lot_size,
                        quote_increment: *tick_size,
                    }
                })
                .collect()
        })
    }

    async fn refresh_market_info(&self) -> Result<Vec<MarketPairHandle>> {
        self.exchange_info
            .refresh(self as &dyn ExchangeInfoRetrieval)
            .await
    }
}
