use crate::coinbase::Coinbase;
use crate::Result;

use crate::coinbase::model::{Product, Trade};

impl Coinbase {
    // That is probably the same as get_exchange_info for binance.const
    pub async fn products(&self) -> Result<Vec<Product>> {
        self.transport.get::<_, ()>("/products", None).await
    }

    pub async fn trades(&self, pair: &str) -> Result<Vec<Trade>> {
        let endpoint = format!("/products/{}/trades", pair);
        self.transport.get::<_, ()>(&endpoint, None).await
    }
}
