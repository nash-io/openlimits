use serde::Deserialize;
use serde::Serialize;

/// This struct represents time in force
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
}