use serde::Deserialize;
use serde::Serialize;
use super::CancelAfter;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "time_in_force")]
pub enum OrderTimeInForce {
    GTC,
    GTT { cancel_after: CancelAfter },
    IOC,
    FOK,
}