use derive_more::Constructor;
use serde::Deserialize;
use serde::Serialize;
use super::OrderStatus;
use super::Paginator;

#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct GetOrderHistoryRequest {
    pub market_pair: Option<String>,
    pub order_status: Option<Vec<OrderStatus>>,
    pub paginator: Option<Paginator>,
}