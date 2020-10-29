use super::BaseClient;
use crate::{
    coinbase::model::{
        Account, CancelAllOrders, CancelOrder, Fill, GetFillsReq, GetOrderRequest, Order,
        OrderRequest, OrderRequestMarketType, OrderRequestType, OrderSide, Paginator, OrderTimeInForce
    },
    exchange_info::MarketPair,
    shared::Result,
};

use rust_decimal::prelude::*;

impl BaseClient {
    pub async fn get_account(&self, paginator: Option<&Paginator>) -> Result<Vec<Account>> {
        self.transport.signed_get("/accounts", paginator).await
    }

    pub async fn get_orders(&self, params: Option<&GetOrderRequest>) -> Result<Vec<Order>> {
        self.transport.signed_get::<_, _>("/orders", params).await
    }

    pub async fn get_order(&self, order_id: String) -> Result<Order> {
        self.transport
            .signed_get::<_, ()>(&format!("/orders/{}", order_id), None)
            .await
    }

    // TODO: refactor buy and sell in order creation in commun function
    pub async fn market_buy(&self, pair: MarketPair, size: Decimal) -> Result<Order> {
        let data = OrderRequest {
            product_id: pair.symbol,
            client_oid: None,
            side: OrderSide::Buy,
            _type: OrderRequestType::Market {
                _type: OrderRequestMarketType::Size {
                    size: size.round_dp(pair.base_increment.normalize().scale()),
                },
            },
            stop: None,
        };

        let transaction = self
            .transport
            .signed_post::<_, (), _>("/orders", None, Some(&data))
            .await?;

        Ok(transaction)
    }

    pub async fn market_sell(&self, pair: MarketPair, size: Decimal) -> Result<Order> {
        let data = OrderRequest {
            product_id: pair.symbol,
            client_oid: None,
            side: OrderSide::Sell,
            _type: OrderRequestType::Market {
                _type: OrderRequestMarketType::Size {
                    size: size.round_dp(pair.base_increment.normalize().scale()),
                },
            },
            stop: None,
        };

        let transaction = self
            .transport
            .signed_post::<_, (), _>("/orders", None, Some(&data))
            .await?;

        Ok(transaction)
    }

    pub async fn limit_buy(
        &self,
        pair: MarketPair,
        size: Decimal,
        price: Decimal,
        time_in_force: OrderTimeInForce,
    ) -> Result<Order> {
        let data = OrderRequest {
            product_id: pair.symbol,
            client_oid: None,
            side: OrderSide::Buy,
            _type: OrderRequestType::Limit {
                size: size.round_dp(pair.base_increment.normalize().scale()),
                price: price.round_dp_with_strategy(
                    pair.quote_increment.normalize().scale(),
                    RoundingStrategy::RoundDown,
                ),
                post_only: true,
                time_in_force: Some(time_in_force),
            },
            stop: None,
        };

        let transaction = self
            .transport
            .signed_post::<_, (), _>("/orders", None, Some(&data))
            .await?;

        Ok(transaction)
    }

    pub async fn limit_sell(
        &self,
        pair: MarketPair,
        size: Decimal,
        price: Decimal,
        time_in_force: OrderTimeInForce,
    ) -> Result<Order> {
        let data = OrderRequest {
            product_id: pair.symbol,
            client_oid: None,
            side: OrderSide::Sell,
            _type: OrderRequestType::Limit {
                size: size.round_dp(pair.base_increment.normalize().scale()),
                price: price.round_dp_with_strategy(
                    pair.quote_increment.normalize().scale(),
                    RoundingStrategy::RoundUp,
                ),
                post_only: true,
                time_in_force: Some(time_in_force),
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

    pub async fn get_fills(&self, params: Option<&GetFillsReq>) -> Result<Vec<Fill>> {
        let resp = self.transport.signed_get::<_, _>("/fills", params).await?;

        Ok(resp)
    }
}
