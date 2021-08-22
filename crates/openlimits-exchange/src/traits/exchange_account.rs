use async_trait::async_trait;
use crate::{
    model::{
        Balance, CancelAllOrdersRequest, CancelOrderRequest, 
        GetOrderHistoryRequest, GetOrderRequest, 
        OpenLimitOrderRequest, OpenMarketOrderRequest, Order,
        OrderCanceled, Paginator, Trade, TradeHistoryRequest,
    },
};
use super::shared::Result;
use crate::model::OrderFilter;

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

    async fn get_open_orders(&self, order_filter: &OrderFilter) -> Result<Vec<Order>> {
        Ok(self
            .get_all_open_orders()
            .await?
            .into_iter()
            .filter(|order| {
                order_filter.client_order_id.as_ref().map(|client_order_id| Some(client_order_id) == order.client_order_id.as_ref()).unwrap_or(true) &&
                order_filter.order_type.as_ref().map(|order_type| *order_type == order.order_type).unwrap_or(true) &&
                order_filter.market_pair.as_ref().map(|market_pair| *market_pair == order.market_pair).unwrap_or(true) &&
                order_filter.side.as_ref().map(|side| *side == order.side).unwrap_or(true) &&
                order_filter.status.as_ref().map(|status| *status == order.status).unwrap_or(true) &&
                order_filter.created_at.as_ref().map(|created_at| order.created_at >= Some(created_at.start) && order.created_at <= Some(created_at.end)).unwrap_or(true) &&
                order_filter.price.as_ref().map(|price| order.price.as_ref() >= Some(&price.start) && order.price.as_ref() <= Some(&price.end)).unwrap_or(true) &&
                order_filter.remaining.as_ref().map(|remaining| order.remaining.as_ref() >= Some(&remaining.start) && order.remaining.as_ref() <= Some(&remaining.end)).unwrap_or(true) &&
                order_filter.size.as_ref().map(|size| order.size >= size.start && order.size <= size.end).unwrap_or(true)
            })
            .collect()
        )
    }
}