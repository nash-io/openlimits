use serde::Deserialize;
use serde::Serialize;
use super::DateRange;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CandleRequestParams {
    #[serde(flatten)]
    pub daterange: Option<DateRange>,
    pub granularity: Option<u32>,
}