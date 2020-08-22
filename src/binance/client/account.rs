use serde_json::json;
use std::collections::HashMap;
use sugar::{convert_args, hashmap};

use crate::{
    binance::{
        model::{
            AccountInformation, AllOrderReq, Balance, Order, OrderCanceled, TradeHistory,
            TradeHistoryReq, Transaction,
        },
        Binance,
    },
    errors::OpenLimitError,
    shared::Result,
};

use rust_decimal::prelude::*;

static ORDER_TYPE_LIMIT: &str = "LIMIT";
static ORDER_TYPE_MARKET: &str = "MARKET";
static ORDER_SIDE_BUY: &str = "BUY";
static ORDER_SIDE_SELL: &str = "SELL";
static TIME_IN_FORCE_GTC: &str = "GTC";

struct OrderRequest {
    pub symbol: String,
    pub qty: Decimal,
    pub price: Decimal,
    pub order_side: String,
    pub order_type: String,
    pub time_in_force: String,
}

impl Binance {
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

    // Check an order's status
    pub async fn order_status(&self, symbol: &str, order_id: u64) -> Result<Order> {
        let params = json! {{"symbol": symbol, "orderId": order_id}};

        let order = self
            .transport
            .signed_get("/api/v3/order", Some(&params))
            .await?;
        Ok(order)
    }

    // Place a LIMIT order - BUY
    pub async fn limit_buy(
        &self,
        symbol: &str,
        qty: Decimal,
        price: Decimal,
    ) -> Result<Transaction> {
        let buy: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty,
            price,
            order_side: ORDER_SIDE_BUY.to_string(),
            order_type: ORDER_TYPE_LIMIT.to_string(),
            time_in_force: TIME_IN_FORCE_GTC.to_string(),
        };
        let params = self.build_order(buy);

        println!("{:?}", params);

        let transaction = self
            .transport
            .signed_post("/api/v3/order", Some(&params))
            .await?;

        Ok(transaction)
    }

    // Place a LIMIT order - SELL
    pub async fn limit_sell(
        &self,
        symbol: &str,
        qty: Decimal,
        price: Decimal,
    ) -> Result<Transaction> {
        let sell: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty,
            price,
            order_side: ORDER_SIDE_SELL.to_string(),
            order_type: ORDER_TYPE_LIMIT.to_string(),
            time_in_force: TIME_IN_FORCE_GTC.to_string(),
        };
        let params = self.build_order(sell);
        let transaction = self
            .transport
            .signed_post("/api/v3/order", Some(&params))
            .await?;

        Ok(transaction)
    }

    // Place a MARKET order - BUY
    pub async fn market_buy(&self, symbol: &str, qty: Decimal) -> Result<Transaction> {
        let buy: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty,
            price: Decimal::from_str("0.0").unwrap(),
            order_side: ORDER_SIDE_BUY.to_string(),
            order_type: ORDER_TYPE_MARKET.to_string(),
            time_in_force: TIME_IN_FORCE_GTC.to_string(),
        };
        let params = self.build_order(buy);
        let transaction = self
            .transport
            .signed_post("/api/v3/order", Some(&params))
            .await?;

        Ok(transaction)
    }

    // Place a MARKET order - SELL
    pub async fn market_sell(&self, symbol: &str, qty: Decimal) -> Result<Transaction> {
        let sell: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty,
            price: Decimal::new(0, 0),
            order_side: ORDER_SIDE_SELL.to_string(),
            order_type: ORDER_TYPE_MARKET.to_string(),
            time_in_force: TIME_IN_FORCE_GTC.to_string(),
        };
        let params = self.build_order(sell);
        let transaction = self
            .transport
            .signed_post("/api/v3/order", Some(&params))
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

    fn build_order(&self, order: OrderRequest) -> HashMap<&'static str, String> {
        let mut params: HashMap<&str, String> = convert_args!(hashmap!(
            "symbol" => order.symbol,
            "side" => order.order_side,
            "type" => order.order_type,
            "quantity" => order.qty.to_string(),
        ));

        if order.price != Decimal::new(0, 0) {
            params.insert("price", order.price.to_string());
            params.insert("timeInForce", order.time_in_force);
        }

        params
    }
}
