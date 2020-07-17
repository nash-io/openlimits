use binance::Binance;
use binance::model::OrderBook as BinanceOrderBookResponse;
use crate::exchange::Exchange;
use crate::model::{OrderBookResponse, OrderBookRequest};
pub struct OpenLimitsBinance; 

impl Exchange for OpenLimitsBinance {
    fn new() -> Binance {
        Binance::new(true)
    }

    fn order_book(&self, req: OrderBookRequest) -> OrderBookResponse {
        self.exchange.get_depth(req);
        Binance::order_book(self, req.as_ref().into()).into()
    }
}

#[derive(Debug, Deserialize, Serialize)]
impl Into<OrderBookRequest> for BinanceOrderBookRequest {
    fn into(self) -> Self {
        BinanceOrderBookRequest
    }
}

#[derive(Debug, Deserialize, Serialize)]
impl From<BinanceOrderBookResponse> for OrderBookResponse {
    fn from(_: BinanceOrderBookResponse) -> Self {
        OrderBookResponse
    }
}
