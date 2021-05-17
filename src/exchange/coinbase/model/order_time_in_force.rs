use serde::Deserialize;
use serde::Serialize;
use super::CancelAfter;

/// This enum represents a time in force order
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "time_in_force")]
pub enum OrderTimeInForce {
    GTC,
    GTT { cancel_after: CancelAfter },
    IOC,
    FOK,
}