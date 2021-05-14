// FIXME: These test cases aren't implemented.
// The purpose of this module is to be sure we have the same functionalities across all the
// supported exchanges.

use dotenv::dotenv;
use nash_native_client::Environment;
use openlimits::exchange::any::{AnyExchange, AnyWsExchange};
use openlimits::exchange::binance::{Binance, BinanceCredentials, BinanceParameters};
use openlimits::exchange::coinbase::client::websocket::CoinbaseWebsocket;
use openlimits::exchange::coinbase::{Coinbase, CoinbaseCredentials, CoinbaseParameters};
use openlimits::OpenLimits;
use openlimits::exchange::traits::stream::OpenLimitsWs;
use openlimits::exchange::nash::{Nash, NashCredentials, NashParameters};
use openlimits::exchange::shared::Result;
use std::env;
use tokio::time::Duration;

#[tokio::test]
async fn account_test() {
    let _exchange = init().await;
}

#[tokio::test]
async fn ws_test() {
    let _websocket = init_ws().await;
}

async fn _nash() -> Result<Nash> {
    let parameters = NashParameters {
        affiliate_code: None,
        credentials: Some(NashCredentials {
            secret: env::var("NASH_API_SECRET").expect("Couldn't get environment variable."),
            session: env::var("NASH_API_KEY").expect("Couldn't get environment variable."),
        }),
        environment: Environment::Sandbox,
        client_id: 1,
        timeout: Duration::from_secs_f32(10.0),
        sign_states_loop_interval: None,
    };
    OpenLimits::instantiate(parameters).await
}

async fn _binance() -> Result<Binance> {
    let parameters = BinanceParameters {
        credentials: Some(BinanceCredentials {
            api_key: env::var("BINANCE_API_KEY").expect("Couldn't get environment variable."),
            api_secret: env::var("BINANCE_API_SECRET").expect("Couldn't get environment variable."),
        }),
        sandbox: true,
    };
    OpenLimits::instantiate(parameters).await
}

async fn coinbase() -> Result<Coinbase> {
    let parameters = CoinbaseParameters {
        sandbox: true,
        credentials: Some(CoinbaseCredentials {
            api_key: env::var("COINBASE_API_KEY").unwrap(),
            api_secret: env::var("COINBASE_API_SECRET").unwrap(),
            passphrase: env::var("COINBASE_PASSPHRASE").unwrap(),
        }),
    };
    OpenLimits::instantiate(parameters).await
}

async fn init() -> Result<AnyExchange> {
    dotenv().ok();
    coinbase().await.map(|exchange| exchange.into())
}

async fn coinbase_websocket() -> OpenLimitsWs<CoinbaseWebsocket> {
    dotenv().ok();

    let websocket = CoinbaseWebsocket::new(CoinbaseParameters {
        sandbox: false,
        credentials: Some(CoinbaseCredentials {
            api_secret: env::var("COINBASE_API_SECRET").unwrap(),
            api_key: env::var("COINBASE_API_KEY").unwrap(),
            passphrase: env::var("COINBASE_PASSPHRASE").unwrap(),
        }),
    });
    OpenLimitsWs { websocket }
}

async fn init_ws() -> AnyWsExchange {
    coinbase_websocket().await.into()
}
