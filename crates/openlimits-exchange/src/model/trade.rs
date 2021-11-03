use derive_more::Constructor;
use rust_decimal::prelude::Decimal;
use serde::Deserialize;
use serde::Serialize;
use super::Liquidity;
use super::Side;

/// This struct represents a trade
#[derive(Serialize, Deserialize, Clone, Constructor, Debug)]
pub struct Trade {
    pub id: String,
    pub buyer_order_id: Option<String>,
    pub seller_order_id: Option<String>,
    pub market_pair: String,
    pub price: Decimal,
    pub qty: Decimal,
    pub fees: Option<Decimal>,
    pub side: Side,
    pub liquidity: Option<Liquidity>,
    pub created_at: String,
}