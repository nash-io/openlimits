use std::fmt;
use crate::model::{OrderBook, OrderBookRequest};

struct OpenLimits<E: Exchange> {
    exchange: E,
}

impl<E: Exchange> OpenLimits<E> {
    pub fn new(exchange: E, sandbox: bool) -> Self {
        Self {
            exchange: exchange::new(book),
        }
    }

    fn order_book(&mut self, req: impl AsRef<OrderBookRequest>) -> OrderBook {
        let resp = self.exchange.order_book(req);

        resp
    }
}

pub trait Exchange {
    type OrderBookRequest: for<'a> From<&'a OrderBook> + fmt::Debug;
    type OrderBook: Into<OrderBook> + fmt::Debug;
    fn new(sandbox: bool) -> Exchange;
    fn order_book(&self, req: impl AsRef<OrderBookRequest>) -> OrderBook;
}
