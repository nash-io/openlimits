use crate::shared::opt_naive_datetime_from_string;
use serde::Deserialize;
use serde::Serialize;
use chrono::naive::NaiveDateTime;

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct DateRange {
    #[serde(with = "opt_naive_datetime_from_string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<NaiveDateTime>,
    #[serde(with = "opt_naive_datetime_from_string")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<NaiveDateTime>,
}