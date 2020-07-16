use binance::Binance;

mod binance {
    use crate::exchange::Exchange;
    pub struct OpenLimitsBinance; 
    
    impl Exchange for OpenLimitsBinance {
        type OrderBookRequest = BinanceOrderBookRequest;
        type OpenLimitsBinance = BinanceOrderBookResponse;
        fn order_book(&self, req: impl AsRef<OrderBookRequest>) -> OrderBookResponse {
            Binance::order_book(self, req.as_ref().into()).into()
        }
    }
    #[derive(Debug, Deserialize, Serialize)]
    pub struct BinanceOrderBookRequest;
    impl<'a> From<&'a OrderBookRequest> for BinanceOrderBookRequest {
        fn from(_: &'a OrderBookRequest) -> Self {
            BinanceOrderBookRequest
        }
    }
    #[derive(Debug, Deserialize, Serialize)]
    pub struct BinanceOrderBookResponse;
    impl From<BinanceOrderBookResponse> for OrderBookResponse {
        fn from(_: BinanceOrderBookResponse) -> Self {
            OrderBookResponse
        }
    }
}
