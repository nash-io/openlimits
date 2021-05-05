use serde::Deserialize;
use serde::Serialize;
use super::DateRange;

/// This struct represents the candle request params
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CandleRequestParams {
    #[serde(flatten)]
    pub daterange: Option<DateRange>,
    pub granularity: Option<u32>,
}