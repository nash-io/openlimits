use super::BaseClient;
use crate::{
    exchange::binance::model::{ExchangeInformation, ServerTime},
    shared::Result,
};
use serde_json::Value;

impl BaseClient {
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
}