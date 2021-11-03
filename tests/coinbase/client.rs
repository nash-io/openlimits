use dotenv::dotenv;
use openlimits::{
    exchange::coinbase::{Coinbase, CoinbaseParameters, CoinbaseWebsocket, CoinbaseCredentials},
    OpenLimits,
};
use openlimits::prelude::*;
use openlimits_exchange::exchange::Environment;

pub async fn init_ws() -> CoinbaseWebsocket {
    CoinbaseWebsocket::new(CoinbaseParameters::production())
        .await
        .expect("Failed to create Binance stream.")
}

pub async fn init() -> Coinbase {
    Coinbase::new(CoinbaseParameters::sandbox())
        .await
        .expect("Failed to create Client")
}

pub async fn init_signed() -> Coinbase {
    dotenv().ok();

    let parameters = CoinbaseParameters {
        credentials: Some(CoinbaseCredentials {
            api_key:    std::env::var("COINBASE_API_KEY").expect("Couldn't get environment variable."),
            api_secret: std::env::var("COINBASE_API_SECRET").expect("Couldn't get environment variable."),
            passphrase: std::env::var("COINBASE_PASSPHRASE").expect("Couldn't get environment variable.")
        }),
        environment: Environment::Sandbox,
    };

    OpenLimits::instantiate(parameters)
        .await
        .expect("Failed to create Client")
}