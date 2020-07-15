use crate::coinbase::model::{
    Account, Order, OrderRequest, OrderRequestMarketType, OrderRequestType, OrderSide,
};
use crate::coinbase::Coinbase;
use crate::Result;

impl Coinbase {
    pub async fn get_account(&self) -> Result<Vec<Account>> {
        self.transport.signed_get::<_, ()>("/accounts", None).await
    }

    pub async fn get_open_orders(&self) -> Result<Vec<Order>> {
        self.transport.signed_get::<_, ()>("/orders", None).await
    }

    pub async fn order_status(&self, order_id: String) -> Result<Order> {
        self.transport
            .signed_get::<_, ()>(&format!("/orders/{}", order_id), None)
            .await
    }
    // TODO: refactor buy and sell in order creation in commun function
    pub async fn market_buy(&self, product: &str, size: f64) -> Result<Order> {
        let params = OrderRequest {
            product_id: product.into(),
            client_oid: None,
            side: OrderSide::Buy,
            _type: OrderRequestType::Market {
                _type: OrderRequestMarketType::Size { size },
            },
            stop: None,
        };

        println!("{:?}", params);

        let transaction = self.transport.signed_post("/orders", Some(&params)).await?;

        Ok(transaction)
    }

    pub async fn market_sell(&self, product: &str, size: f64) -> Result<Order> {
        let params = OrderRequest {
            product_id: product.into(),
            client_oid: None,
            side: OrderSide::Sell,
            _type: OrderRequestType::Market {
                _type: OrderRequestMarketType::Size { size },
            },
            stop: None,
        };

        println!("{:?}", params);

        let transaction = self.transport.signed_post("/orders", Some(&params)).await?;

        Ok(transaction)
    }

    pub async fn limit_buy(&self, product: &str, size: f64, price: f64) -> Result<Order> {
        let params = OrderRequest {
            product_id: product.into(),
            client_oid: None,
            side: OrderSide::Buy,
            _type: OrderRequestType::Limit {
                price,
                size,
                post_only: true,
                time_in_force: None
            },
            stop: None,
        };

        println!("{:?}", params);

        let transaction = self.transport.signed_post("/orders", Some(&params)).await?;

        Ok(transaction)
    }

    pub async fn limit_sell(&self, product: &str, size: f64, price: f64) -> Result<Order> {
        let params = OrderRequest {
            product_id: product.into(),
            client_oid: None,
            side: OrderSide::Sell,
            _type: OrderRequestType::Limit {
                price,
                size,
                post_only: true,
                time_in_force: None
            },
            stop: None,
        };

        println!("{:?}", params);

        let transaction = self.transport.signed_post("/orders", Some(&params)).await?;

        Ok(transaction)
    }

}
