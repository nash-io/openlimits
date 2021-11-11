use dotenv::dotenv;
use openlimits::{
    exchange::nash::{Nash, NashParameters, NashWebsocket, NashCredentials},
    OpenLimits,
};
use openlimits::prelude::*;
use openlimits_exchange::exchange::Environment;
use std::time::Duration;

pub async fn init_ws() -> NashWebsocket {
    NashWebsocket::new(NashParameters::production())
        .await
        .expect("Failed to create Binance stream.")
}

pub async fn init() -> Nash {
    Nash::new(NashParameters::sandbox())
        .await
        .expect("Failed to create Client")
}

pub async fn init_signed() -> Nash {
    dotenv().ok();

    let parameters = NashParameters {
        credentials: Some(NashCredentials {
            api_key:    std::env::var("NASH_API_KEY").expect("Couldn't get environment variable."),
            api_secret: std::env::var("NASH_API_SECRET").expect("Couldn't get environment variable."),
        }),
        environment: Environment::Sandbox,
        affiliate_code: None,
        client_id: 1,
        sign_states_loop_interval: None,
        timeout: Duration::new(10, 0)
    };

    OpenLimits::instantiate(parameters)
        .await
        .expect("Failed to create Client")
}