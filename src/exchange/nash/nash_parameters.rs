use tokio::time::Duration;
pub use nash_native_client::{Client, Environment};
use super::NashCredentials;

pub struct NashParameters {
    pub affiliate_code: Option<String>,
    pub credentials: Option<NashCredentials>,
    pub client_id: u64,
    pub environment: Environment,
    pub timeout: Duration,
    pub sign_states_loop_interval: Option<Duration>,
}

impl Clone for NashParameters {
    fn clone(&self) -> Self {
        NashParameters {
            affiliate_code: self.affiliate_code.clone(),
            credentials: self.credentials.clone(),
            client_id: self.client_id,
            environment: match self.environment {
                Environment::Production => Environment::Production,
                Environment::Sandbox => Environment::Sandbox,
                Environment::Dev(s) => Environment::Dev(s),
            },
            timeout: self.timeout,
            sign_states_loop_interval: self.sign_states_loop_interval,
        }
    }
}