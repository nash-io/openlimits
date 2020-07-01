use crate::Result;
use serde_json::Value;

use crate::binance::client::Binance;
use crate::binance::model::{ExchangeInfo, ExchangeInformation, ServerTime};

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

    pub async fn get_exchange_info(&self) -> Result<ExchangeInfo> {
        self.transport
            .get::<_, ()>("/api/v1/exchangeInfo", None)
            .await
    }

    // Obtain exchange information (rate limits, symbol metadata etc)
    pub async fn exchange_info(&self) -> Result<ExchangeInformation> {
        self.transport
            .get::<_, ()>("/api/v1/exchangeInfo", None)
            .await
    }
}
