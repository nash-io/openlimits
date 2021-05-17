use tokio::time::Duration;
pub use nash_native_client::{Client, Environment};
use super::NashCredentials;

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
