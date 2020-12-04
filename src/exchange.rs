use async_trait::async_trait;

use crate::{
    model::{
        Balance, CancelAllOrdersRequest, CancelOrderRequest, Candle, GetHistoricRatesRequest,
        GetHistoricTradesRequest, GetOrderHistoryRequest, GetOrderRequest, GetPriceTickerRequest,
        OpenLimitOrderRequest, OpenMarketOrderRequest, Order, OrderBookRequest, OrderBookResponse,
        OrderCanceled, Paginator, Ticker, Trade, TradeHistoryRequest,
    },
    shared::Result,
};
use crate::exchange_info::ExchangeInfoRetrieval;

pub struct OpenLimits {}

impl OpenLimits {
    pub async fn instantiate<E: Exchange>(parameters: E::InitParams) -> E {
        E::new(parameters).await
    }
}

#[async_trait]
pub trait Exchange: ExchangeInfoRetrieval + ExchangeAccount + ExchangeMarketData {
    type InitParams;
    type InnerClient;
    async fn new(params: Self::InitParams) -> Self;
    fn inner_client(&self) -> Option<&Self::InnerClient>;
}

#[async_trait]
pub trait ExchangeMarketData {
    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse>;
    async fn get_price_ticker(&self, req: &GetPriceTickerRequest) -> Result<Ticker>;
    async fn get_historic_rates(&self, req: &GetHistoricRatesRequest) -> Result<Vec<Candle>>;
    async fn get_historic_trades(&self, req: &GetHistoricTradesRequest) -> Result<Vec<Trade>>;
}

#[async_trait]
pub trait ExchangeAccount {
    async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order>;
    async fn limit_sell(&self, req: &OpenLimitOrderRequest) -> Result<Order>;
    async fn market_buy(&self, req: &OpenMarketOrderRequest) -> Result<Order>;
    async fn market_sell(&self, req: &OpenMarketOrderRequest) -> Result<Order>;
    async fn cancel_order(&self, req: &CancelOrderRequest) -> Result<OrderCanceled>;
    async fn cancel_all_orders(&self, req: &CancelAllOrdersRequest) -> Result<Vec<OrderCanceled>>;
    async fn get_all_open_orders(&self) -> Result<Vec<Order>>;
    async fn get_order_history(&self, req: &GetOrderHistoryRequest) -> Result<Vec<Order>>;
    async fn get_trade_history(&self, req: &TradeHistoryRequest) -> Result<Vec<Trade>>;
    async fn get_account_balances(&self, paginator: Option<Paginator>) -> Result<Vec<Balance>>;
    async fn get_order(&self, req: &GetOrderRequest) -> Result<Order>;
}
