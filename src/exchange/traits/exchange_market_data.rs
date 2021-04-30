use async_trait::async_trait;
use crate::{
    model::{
        Candle, GetHistoricRatesRequest,
        GetHistoricTradesRequest, GetPriceTickerRequest,
        OrderBookRequest, OrderBookResponse,
        Ticker, Trade,
    },
};
use super::shared::Result;

#[async_trait]
pub trait ExchangeMarketData {
    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse>;
    async fn get_price_ticker(&self, req: &GetPriceTickerRequest) -> Result<Ticker>;
    async fn get_historic_rates(&self, req: &GetHistoricRatesRequest) -> Result<Vec<Candle>>;
    async fn get_historic_trades(&self, req: &GetHistoricTradesRequest) -> Result<Vec<Trade>>;
}