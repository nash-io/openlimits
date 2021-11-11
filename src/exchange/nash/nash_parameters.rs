use tokio::time::Duration;
pub use nash_native_client::Client;
use super::NashCredentials;
use openlimits_exchange::exchange::Environment;

/// This struct represents the parameters
#[derive(Clone)]
pub struct NashParameters {
    pub affiliate_code: Option<String>,
    pub credentials: Option<NashCredentials>,
    pub client_id: u64,
    pub environment: Environment,
    pub timeout: Duration,
    pub sign_states_loop_interval: Option<Duration>,
}

impl NashParameters {
    pub fn production() -> Self {
        Self {
            affiliate_code: None,
            credentials: None,
            client_id: 1,
            environment: Environment::Production,
            timeout: Duration::new(10, 0),
            sign_states_loop_interval: None
        }
    }

    pub fn sandbox() -> Self {
        Self {
            affiliate_code: None,
            credentials: None,
            client_id: 1,
            environment: Environment::Sandbox,
            timeout: Duration::new(10, 0),
            sign_states_loop_interval: None
        }
    }
}
