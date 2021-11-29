use dotenv::dotenv;
use openlimits::{
    exchange::binance::{Binance, BinanceParameters, BinanceWebsocket, BinanceCredentials},
    OpenLimits,
    exchange::traits::{
        Exchange,
        stream::ExchangeStream
    }
};
use openlimits_exchange::exchange::Environment;

pub async fn init_ws() -> BinanceWebsocket {
    BinanceWebsocket::new(BinanceParameters::production())
        .await
        .expect("Failed to create Binance stream.")
}

pub async fn init() -> Binance {
    Binance::new(BinanceParameters::sandbox())
        .await
        .expect("Failed to create Client")
}

pub async fn init_signed() -> Binance {
    dotenv().ok();

    let parameters = BinanceParameters {
        credentials: Some(BinanceCredentials {
            api_key:    std::env::var("BINANCE_API_KEY").expect("Couldn't get environment variable."),
            api_secret: std::env::var("BINANCE_API_SECRET").expect("Couldn't get environment variable."),
        }),
        environment: Environment::Sandbox,
    };

    OpenLimits::instantiate(parameters)
        .await
        .expect("Failed to create Client")
}