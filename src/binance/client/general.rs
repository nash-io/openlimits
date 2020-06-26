use serde_json::Value;
use crate::Result;

use crate::binance::client::Binance;
use crate::binance::model::{ExchangeInfo, ExchangeInformation, ServerTime};

impl Binance {
    // Test connectivity
    pub async fn ping(&self) -> Result<String> {
        Ok(self
            .transport
            .get::<_, ()>("/api/v1/ping", None).await?
            .map(|_: Value| "pong".into()))
    }

    // Check server time
    pub async fn get_server_time(&self) -> Result<ServerTime> {
        Ok(self.transport.get::<_, ()>("/api/v1/time", None).await?)
    }

    pub async fn get_exchange_info(&self) -> Result<ExchangeInfo> {
        Ok(self.transport.get::<_, ()>("/api/v1/exchangeInfo", None).await?)
    }

    // Obtain exchange information (rate limits, symbol metadata etc)
    pub async fn exchange_info(
        &self,
    ) ->  Result<ExchangeInformation> {
        let info = self.transport.get::<_, ()>("/api/v1/exchangeInfo", None).await?;
        Ok(info)
    }
}
