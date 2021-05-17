use serde::Deserialize;
use serde::Serialize;
use super::Interval;
use super::RateLimitType;

/// This struct represents the rate limit
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    rate_limit_type: RateLimitType,
    interval: Interval,
    limit: u64,
}