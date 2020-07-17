use crate::model::{OrderBookRequest, OrderBookResponse};

struct OpenLimits<E: Exchange> {
    exchange: E,
}

impl<E: Exchange> OpenLimits<E> {
    pub fn new(sandbox: bool) -> Self {
        Self {
            exchange: Exchange::new(sandbox),
        }
    }

    pub fn order_book(&mut self, req: impl AsRef<OrderBookRequest>) -> OrderBookResponse {
        self.exchange.order_book(req)
    }
}

pub trait Exchange {
    fn new(sandbox: bool) -> Self;
    fn order_book(&mut self, req: impl AsRef<OrderBookRequest>) -> OrderBookResponse;
}
