use crate::shared::naive_datetime_from_string;
use serde::Deserialize;
use serde::Serialize;
use chrono::naive::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "time_in_force")]
pub enum OrderTimeInForceResponse {
    GTC,
    GTT {
        #[serde(with = "naive_datetime_from_string")]
        expire_time: NaiveDateTime,
    },
    IOC,
    FOK,
}