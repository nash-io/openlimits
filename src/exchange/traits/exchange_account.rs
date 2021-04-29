use async_trait::async_trait;
use crate::{
    model::{
        Balance, CancelAllOrdersRequest, CancelOrderRequest, 
        GetOrderHistoryRequest, GetOrderRequest, 
        OpenLimitOrderRequest, OpenMarketOrderRequest, Order,
        OrderCanceled, Paginator, Trade, TradeHistoryRequest,
    },
};
use crate::shared::Result;

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