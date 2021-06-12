use derive_more::Constructor;
use serde::Deserialize;
use serde::Serialize;
use crate::model::{OrderStatus, Paginator};

/// This struct represents the historic of the orders
#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct GetOrderHistoryRequest {
    pub market_pair: Option<String>,
    pub order_status: Option<Vec<OrderStatus>>,
    pub paginator: Option<Paginator>,
}