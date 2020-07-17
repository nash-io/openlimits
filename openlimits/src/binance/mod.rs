use derive_more::{Deref, DerefMut};

use crate::exchange::Exchange;
use crate::model::{OrderBookRequest, OrderBookResponse};

#[derive(Deref, DerefMut)]
pub struct Binance(binance::Binance);

impl Exchange for Binance {
    fn new(sandbox: bool) -> Binance {
        Binance(binance::Binance::new(sandbox))
    }

    fn order_book(&mut self, req: impl AsRef<OrderBookRequest>) -> OrderBookResponse {
        Default::default()
    }
}
