use serde_json::json;
use std::collections::HashMap;

use crate::{
    binance::{
        model::{
            AccountInformation, AllOrderReq, Balance, Order, OrderCanceled, OrderRequest,
            TradeHistory, TradeHistoryReq, ORDER_SIDE_BUY, ORDER_SIDE_SELL, ORDER_TYPE_LIMIT,
            ORDER_TYPE_MARKET, TIME_IN_FORCE_GTC,
        },
    },
    exchange_info::MarketPair,
    errors::OpenLimitError,
    shared::Result,
};
use super::BaseClient;
use rust_decimal::prelude::*;

impl BaseClient {
    // Account Information
    pub async fn get_account(&self) -> Result<AccountInformation> {
        let account_info = self
            .transport
            .signed_get::<_, ()>("/api/v3/account", None)
            .await?;

        Ok(account_info)
    }

    // Balance for ONE Asset
    pub async fn get_balance(&self, asset: &str) -> Result<Balance> {
        let asset = asset.to_string();
        let search = move |account: AccountInformation| -> Result<Balance> {
            let balance = account
                .balances
                .into_iter()
                .find(|balance| balance.asset == asset)
                .ok_or(OpenLimitError::AssetNotFound())?;

            Ok(balance)
        };

        self.get_account().await.and_then(search)
    }

    // Current open orders for ONE symbol
    pub async fn get_open_orders(&self, symbol: &str) -> Result<Vec<Order>> {
        let params: HashMap<&str, String> =
            [("symbol", String::from(symbol))].iter().cloned().collect();
        let orders = self
            .transport
            .signed_get("/api/v3/openOrders", Some(&params))
            .await?;
        Ok(orders)
    }

    // All current open orders
    pub async fn get_all_open_orders(&self) -> Result<Vec<Order>> {
        let orders = self
            .transport
            .signed_get::<_, ()>("/api/v3/openOrders", None)
            .await?;
        Ok(orders)
    }

    pub async fn get_all_orders(&self, params: &AllOrderReq) -> Result<Vec<Order>> {
        let orders = self
            .transport
            .signed_get("/api/v3/allOrders", Some(params))
            .await?;
        Ok(orders)
    }

    pub async fn get_order(&self, symbol: &str, order_id: u64) -> Result<Order> {
        let params = json! {{"symbol": symbol, "orderId": order_id}};

        let order = self
            .transport
            .signed_get("/api/v3/order", Some(&params))
            .await?;
        Ok(order)
    }

    // Place a LIMIT order - BUY
    pub async fn limit_buy(&self, symbol: &str, pair: MarketPair, qty: Decimal, price: Decimal) -> Result<Order> {
        let buy: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            quantity: qty.round_dp(pair.base_increment.normalize().scale()),
            price: Some(price.round_dp_with_strategy(
                pair.quote_increment.normalize().scale(),
                RoundingStrategy::RoundDown,
            )),
            order_side: ORDER_SIDE_BUY.to_string(),
            order_type: ORDER_TYPE_LIMIT.to_string(),
            time_in_force: Some(TIME_IN_FORCE_GTC.to_string()),
        };

        let transaction = self
            .transport
            .signed_post("/api/v3/order", Some(&buy))
            .await?;

        Ok(transaction)
    }

    // Place a LIMIT order - SELL

    pub async fn limit_sell(&self, symbol: &str, pair: MarketPair, qty: Decimal, price: Decimal) -> Result<Order> {
        let sell: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            quantity: qty.round_dp(pair.base_increment.normalize().scale()),
            price: Some(price.round_dp_with_strategy(
                pair.quote_increment.normalize().scale(),
                RoundingStrategy::RoundUp,
            )),
            order_side: ORDER_SIDE_SELL.to_string(),
            order_type: ORDER_TYPE_LIMIT.to_string(),
            time_in_force: Some(TIME_IN_FORCE_GTC.to_string()),
        };

        let transaction = self
            .transport
            .signed_post("/api/v3/order", Some(&sell))
            .await?;

        Ok(transaction)
    }

    // Place a MARKET order - BUY
    pub async fn market_buy(&self, symbol: &str, pair: MarketPair, qty: Decimal) -> Result<Order> {
        let buy: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            quantity: qty.round_dp(pair.base_increment.normalize().scale()),
            price: None,
            order_side: ORDER_SIDE_BUY.to_string(),
            order_type: ORDER_TYPE_MARKET.to_string(),
            time_in_force: None,
        };

        let transaction = self
            .transport
            .signed_post("/api/v3/order", Some(&buy))
            .await?;

        Ok(transaction)
    }

    // Place a MARKET order - SELL
    pub async fn market_sell(&self, symbol: &str, pair: MarketPair, qty: Decimal) -> Result<Order> {
        let sell: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            quantity: qty.round_dp(pair.base_increment.normalize().scale()),
            price: None,
            order_side: ORDER_SIDE_SELL.to_string(),
            order_type: ORDER_TYPE_MARKET.to_string(),
            time_in_force: None,
        };

        let transaction = self
            .transport
            .signed_post("/api/v3/order", Some(&sell))
            .await?;
        Ok(transaction)
    }

    // Check an order's status
    pub async fn cancel_order(&self, symbol: &str, order_id: u64) -> Result<OrderCanceled> {
        let params = json! {{"symbol":symbol, "orderId":order_id}};
        let order_canceled = self
            .transport
            .signed_delete("/api/v3/order", Some(&params))
            .await?;
        Ok(order_canceled)
    }

    pub async fn cancel_all_orders(&self, symbol: &str) -> Result<Vec<OrderCanceled>> {
        let params = json! {{"symbol":symbol}};
        let orders_canceled = self
            .transport
            .signed_delete("/api/v3/openOrders", Some(&params))
            .await?;
        Ok(orders_canceled)
    }

    // Trade history
    pub async fn trade_history(&self, params: &TradeHistoryReq) -> Result<Vec<TradeHistory>> {
        let trade_history = self
            .transport
            .signed_get("/api/v3/myTrades", Some(params))
            .await?;

        Ok(trade_history)
    }
}
