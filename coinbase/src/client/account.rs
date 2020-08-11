use crate::model::{
    Account, CancelAllOrders, CancelOrder, Fill, Order, OrderRequest, OrderRequestMarketType,
    OrderRequestType, OrderSide,
};
use crate::Coinbase;

use shared::Result;

use rust_decimal::prelude::Decimal;
use serde_json::json;

impl Coinbase {
    pub async fn get_account(&self) -> Result<Vec<Account>> {
        self.transport.signed_get::<_, ()>("/accounts", None).await
    }

    pub async fn get_all_open_orders(&self) -> Result<Vec<Order>> {
        self.transport.signed_get::<_, ()>("/orders", None).await
    }

    pub async fn get_all_orders<'a>(&self, product_id: Option<&str>) -> Result<Vec<Order>> {
        let mut params = json! {{"status": "all"}};

        if let Some(p) = product_id {
            params["product_id"] = json!(p);
        }

        self.transport
            .signed_get::<_, _>("/orders", Some(&params))
            .await
    }

    pub async fn order_status(&self, order_id: String) -> Result<Order> {
        self.transport
            .signed_get::<_, ()>(&format!("/orders/{}", order_id), None)
            .await
    }
    // TODO: refactor buy and sell in order creation in commun function
    pub async fn market_buy(&self, product: &str, size: Decimal) -> Result<Order> {
        let data = OrderRequest {
            product_id: product.into(),
            client_oid: None,
            side: OrderSide::Buy,
            _type: OrderRequestType::Market {
                _type: OrderRequestMarketType::Size { size },
            },
            stop: None,
        };

        let transaction = self
            .transport
            .signed_post::<_, (), _>("/orders", None, Some(&data))
            .await?;

        Ok(transaction)
    }

    pub async fn market_sell(&self, product: &str, size: Decimal) -> Result<Order> {
        let data = OrderRequest {
            product_id: product.into(),
            client_oid: None,
            side: OrderSide::Sell,
            _type: OrderRequestType::Market {
                _type: OrderRequestMarketType::Size { size },
            },
            stop: None,
        };

        let transaction = self
            .transport
            .signed_post::<_, (), _>("/orders", None, Some(&data))
            .await?;

        Ok(transaction)
    }

    pub async fn limit_buy(&self, product: &str, size: Decimal, price: Decimal) -> Result<Order> {
        let data = OrderRequest {
            product_id: product.into(),
            client_oid: None,
            side: OrderSide::Buy,
            _type: OrderRequestType::Limit {
                price,
                size,
                post_only: true,
                time_in_force: None,
            },
            stop: None,
        };

        let transaction = self
            .transport
            .signed_post::<_, (), _>("/orders", None, Some(&data))
            .await?;

        Ok(transaction)
    }

    pub async fn limit_sell(&self, product: &str, size: Decimal, price: Decimal) -> Result<Order> {
        let data = OrderRequest {
            product_id: product.into(),
            client_oid: None,
            side: OrderSide::Sell,
            _type: OrderRequestType::Limit {
                price,
                size,
                post_only: true,
                time_in_force: None,
            },
            stop: None,
        };

        let transaction = self
            .transport
            .signed_post::<_, (), _>("/orders", None, Some(&data))
            .await?;

        Ok(transaction)
    }

    pub async fn cancel_order(&self, order_id: String, product_id: Option<&str>) -> Result<String> {
        let params = if let Some(product_id) = product_id {
            CancelOrder {
                product_id: Some(String::from(product_id)),
            }
        } else {
            CancelOrder { product_id: None }
        };

        let path = format!("/orders/{}", order_id);
        let resp = self
            .transport
            .signed_delete::<_, _, ()>(&path, Some(&params), None)
            .await?;

        Ok(resp)
    }

    pub async fn cancel_all_orders(&self, product_id: Option<&str>) -> Result<Vec<String>> {
        let params = if let Some(product_id) = product_id {
            CancelAllOrders {
                product_id: Some(String::from(product_id)),
            }
        } else {
            CancelAllOrders { product_id: None }
        };

        let resp = self
            .transport
            .signed_delete::<_, _, ()>("/orders", Some(&params), None)
            .await?;

        Ok(resp)
    }

    pub async fn get_fills_for_order(&self, order_id: &str) -> Result<Vec<Fill>> {
        let params = json! {{"order_id":order_id}};

        let resp = self
            .transport
            .signed_get::<_, _>("/fills", Some(&params))
            .await?;

        Ok(resp)
    }

    pub async fn get_fills_for_product(&self, product_id: &str) -> Result<Vec<Fill>> {
        let params = json! {{"product_id":product_id}};

        let resp = self
            .transport
            .signed_get::<_, _>("/fills", Some(&params))
            .await?;

        Ok(resp)
    }
}
