use serde_json::json;
use std::collections::HashMap;
use sugar::{convert_args, hashmap};

use crate::binance::Binance;
use crate::binance::model::{
    AccountInformation, Balance, Order, OrderCanceled, TradeHistory, Transaction,
};
use crate::errors::OpenLimitError;
use crate::Result;

static ORDER_TYPE_LIMIT: &str = "LIMIT";
static ORDER_TYPE_MARKET: &str = "MARKET";
static ORDER_SIDE_BUY: &str = "BUY";
static ORDER_SIDE_SELL: &str = "SELL";
static TIME_IN_FORCE_GTC: &str = "GTC";

static API_V3_ORDER: &str = "/api/v3/order";

struct OrderRequest {
    pub symbol: String,
    pub qty: f64,
    pub price: f64,
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
        let params: HashMap<&str, String> = [("symbol", String::from(symbol))].iter().cloned().collect();
        let orders = self
            .transport
            .signed_get("/api/v3/openOrders", Some(params))
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

    // Check an order's status
    pub async fn order_status(&self, symbol: &str, order_id: u64) -> Result<Order> {
        let params = json! {{"symbol": symbol, "orderId": order_id}};

        let order = self
            .transport
            .signed_get(API_V3_ORDER, Some(params))
            .await?;
        Ok(order)
    }

    // Place a LIMIT order - BUY
    pub async fn limit_buy(&self, symbol: &str, qty: f64, price: f64) -> Result<Transaction> {
        let buy: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty,
            price,
            order_side: ORDER_SIDE_BUY.to_string(),
            order_type: ORDER_TYPE_LIMIT.to_string(),
            time_in_force: TIME_IN_FORCE_GTC.to_string(),
        };
        let params = self.build_order(buy);

        let transaction = self
            .transport
            .signed_post(API_V3_ORDER, Some(params))
            .await?;

        Ok(transaction)
    }

    // Place a LIMIT order - SELL
    pub async fn limit_sell(&self, symbol: &str, qty: f64, price: f64) -> Result<Transaction> {
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
            .signed_post(API_V3_ORDER, Some(params))
            .await?;

        Ok(transaction)
    }

    // Place a MARKET order - BUY
    pub async fn market_buy(&self, symbol: &str, qty: f64) -> Result<Transaction> {
        let buy: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty: qty,
            price: 0.0,
            order_side: ORDER_SIDE_BUY.to_string(),
            order_type: ORDER_TYPE_MARKET.to_string(),
            time_in_force: TIME_IN_FORCE_GTC.to_string(),
        };
        let params = self.build_order(buy);
        let transaction = self
            .transport
            .signed_post(API_V3_ORDER, Some(params))
            .await?;

        Ok(transaction)
    }

    // Place a MARKET order - SELL
    pub async fn market_sell(&self, symbol: &str, qty: f64) -> Result<Transaction> {
        let sell: OrderRequest = OrderRequest {
            symbol: symbol.into(),
            qty,
            price: 0.0,
            order_side: ORDER_SIDE_SELL.to_string(),
            order_type: ORDER_TYPE_MARKET.to_string(),
            time_in_force: TIME_IN_FORCE_GTC.to_string(),
        };
        let params = self.build_order(sell);
        let transaction = self
            .transport
            .signed_post(API_V3_ORDER, Some(params))
            .await?;
        Ok(transaction)
    }

    // Check an order's status
    pub async fn cancel_order(&self, symbol: &str, order_id: u64) -> Result<OrderCanceled> {
        let params = json! {{"symbol":symbol, "orderId":order_id}};
        let order_canceled = self
            .transport
            .signed_delete(API_V3_ORDER, Some(params))
            .await?;
        Ok(order_canceled)
    }

    // Trade history
    pub async fn trade_history(&self, symbol: &str) -> Result<Vec<TradeHistory>> {
        let params = json! {{"symbol":symbol}};
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

        if order.price != 0.0 {
            params.insert("price", order.price.to_string());
            params.insert("timeInForce", order.time_in_force);
        }

        params
    }
}
