use serde::Deserialize;
use serde::Serialize;

/// This enum represents an exchange filter
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "filterType", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExchangeFilter {
    #[serde(rename_all = "camelCase")]
    ExchangeMaxNumOrders { max_num_orders: u64 },
    #[serde(rename_all = "camelCase")]
    ExchangeMaxNumAlgoOrders { max_num_algo_orders: u64 },
}
