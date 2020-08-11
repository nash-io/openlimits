use async_trait::async_trait;
use derive_more::{Deref, DerefMut};
use shared::Result;

use crate::exchange::Exchange;
use crate::model::{
    Asks, Balance, Bids, CancelAllOrdersRequest, CancelOrderRequest, GetPriceTickerRequest,
    Liquidity, OpenLimitOrderRequest, OpenMarketOrderRequest, Order, OrderBookRequest,
    OrderBookResponse, OrderCanceled, Side, Ticker, Trade, TradeHistoryRequest,
};
use shared::errors::OpenLimitError;

#[derive(Deref, DerefMut)]
pub struct Coinbase(coinbase::Coinbase);

impl Coinbase {
    pub fn new(sandbox: bool) -> Self {
        Coinbase(coinbase::Coinbase::new(sandbox))
    }

    pub fn with_credential(
        api_key: &str,
        api_secret: &str,
        passphrase: &str,
        sandbox: bool,
    ) -> Self {
        Coinbase(coinbase::Coinbase::with_credential(
            api_key, api_secret, passphrase, sandbox,
        ))
    }
}

#[async_trait]
impl Exchange for Coinbase {
    type OrderIdType = String;
    type TradeIdType = u64;
    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse> {
        self.book::<coinbase::model::BookRecordL2>(&req.symbol)
            .await
            .map(Into::into)
    }

    async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self::OrderIdType>> {
        coinbase::Coinbase::limit_buy(self, &req.symbol, req.size, req.price)
            .await
            .map(Into::into)
    }

    async fn limit_sell(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self::OrderIdType>> {
        coinbase::Coinbase::limit_sell(self, &req.symbol, req.size, req.price)
            .await
            .map(Into::into)
    }

    async fn market_buy(&self, req: &OpenMarketOrderRequest) -> Result<Order<Self::OrderIdType>> {
        coinbase::Coinbase::market_buy(self, &req.symbol, req.size)
            .await
            .map(Into::into)
    }

    async fn market_sell(&self, req: &OpenMarketOrderRequest) -> Result<Order<Self::OrderIdType>> {
        coinbase::Coinbase::market_sell(self, &req.symbol, req.size)
            .await
            .map(Into::into)
    }

    async fn cancel_order(
        &self,
        req: &CancelOrderRequest<Self::OrderIdType>,
    ) -> Result<OrderCanceled<Self::OrderIdType>> {
        coinbase::Coinbase::cancel_order(self, req.id.clone(), req.pair.as_deref())
            .await
            .map(Into::into)
    }

    async fn cancel_all_orders(
        &self,
        req: &CancelAllOrdersRequest,
    ) -> Result<Vec<OrderCanceled<Self::OrderIdType>>> {
        coinbase::Coinbase::cancel_all_orders(self, req.pair.as_deref())
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_all_open_orders(&self) -> Result<Vec<Order<Self::OrderIdType>>> {
        coinbase::Coinbase::get_all_open_orders(self)
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_order_history(
        &self,
        req: &crate::model::GetOrderHistoryRequest,
    ) -> Result<Vec<Order<Self::OrderIdType>>> {
        coinbase::Coinbase::get_all_orders(self, req.symbol.as_deref())
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_account_balances(&self) -> Result<Vec<Balance>> {
        coinbase::Coinbase::get_account(self)
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_trade_history(
        &self,
        req: &TradeHistoryRequest<Self::OrderIdType>,
    ) -> Result<Vec<Trade<Self::TradeIdType, Self::OrderIdType>>> {
        if let Some(order_id) = req.order_id.as_ref() {
            coinbase::Coinbase::get_fills_for_order(self, order_id.as_ref())
                .await
                .map(|v| v.into_iter().map(Into::into).collect())
        } else if let Some(product_id) = req.pair.as_ref() {
            coinbase::Coinbase::get_fills_for_product(self, product_id.as_ref())
                .await
                .map(|v| v.into_iter().map(Into::into).collect())
        } else {
            Err(OpenLimitError::MissingParameter(
                "One of order_id or pair parameter is required.".to_string(),
            ))
        }
    }

    async fn get_price_ticker(&self, req: &GetPriceTickerRequest) -> Result<Ticker> {
        coinbase::Coinbase::ticker(self, &req.symbol)
            .await
            .map(Into::into)
    }
}

impl From<coinbase::model::Book<coinbase::model::BookRecordL2>> for OrderBookResponse {
    fn from(book: coinbase::model::Book<coinbase::model::BookRecordL2>) -> Self {
        Self {
            last_update_id: None,
            bids: book.bids.into_iter().map(Into::into).collect(),
            asks: book.asks.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<coinbase::model::BookRecordL2> for Bids {
    fn from(bids: coinbase::model::BookRecordL2) -> Self {
        Self {
            price: bids.price,
            qty: bids.size,
        }
    }
}

impl From<coinbase::model::BookRecordL2> for Asks {
    fn from(bids: coinbase::model::BookRecordL2) -> Self {
        Self {
            price: bids.price,
            qty: bids.size,
        }
    }
}

impl From<coinbase::model::Order> for Order<String> {
    fn from(order: coinbase::model::Order) -> Self {
        Self {
            id: order.id,
            symbol: order.product_id,
            client_order_id: None,
            created_at: order.created_at.into(),
        }
    }
}

impl From<String> for OrderCanceled<String> {
    fn from(id: String) -> Self {
        Self { id }
    }
}

impl From<coinbase::model::Account> for Balance {
    fn from(account: coinbase::model::Account) -> Self {
        Self {
            asset: account.currency,
            free: account.available,
            total: account.balance,
        }
    }
}

impl From<coinbase::model::Fill> for Trade<u64, String> {
    fn from(fill: coinbase::model::Fill) -> Self {
        Self {
            id: fill.trade_id,
            order_id: fill.order_id,
            pair: fill.product_id,
            price: fill.price,
            qty: fill.size,
            fees: fill.fee,
            side: match fill.side.as_str() {
                "buy" => Side::Buy,
                _ => Side::Sell,
            },
            liquidity: match fill.liquidity.as_str() {
                "M" => Some(Liquidity::Maker),
                "T" => Some(Liquidity::Taker),
                _ => None,
            },
            created_at: fill.created_at.into(),
        }
    }
}

impl From<coinbase::model::Ticker> for Ticker {
    fn from(ticker: coinbase::model::Ticker) -> Self {
        Self {
            price: ticker.price,
        }
    }
}
