use async_trait::async_trait;
use derive_more::Constructor;

use crate::{
    exchange_info::MarketPairHandle,
    model::{
        Balance, CancelAllOrdersRequest, CancelOrderRequest, Candle, GetHistoricRatesRequest,
        GetHistoricTradesRequest, GetOrderHistoryRequest, GetOrderRequest, GetPriceTickerRequest,
        OpenLimitOrderRequest, OpenMarketOrderRequest, Order, OrderBookRequest, OrderBookResponse,
        OrderCanceled, Paginator, Ticker, Trade, TradeHistoryRequest,
    },
    shared::Result,
};

#[derive(Constructor)]
pub struct OpenLimits<E: Exchange> {
    pub exchange: E,
}

impl<E: Exchange> OpenLimits<E> {
    pub async fn refresh_market_info(&self) -> Result<Vec<MarketPairHandle>> {
        self.exchange.refresh_market_info().await
    }

    pub async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse> {
        self.exchange.order_book(req).await
    }

    pub async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order<E::OrderIdType>> {
        self.exchange.limit_buy(req).await
    }

    pub async fn limit_sell(&self, req: &OpenLimitOrderRequest) -> Result<Order<E::OrderIdType>> {
        self.exchange.limit_sell(req).await
    }

    pub async fn market_buy(&self, req: &OpenMarketOrderRequest) -> Result<Order<E::OrderIdType>> {
        self.exchange.market_buy(req).await
    }

    pub async fn market_sell(&self, req: &OpenMarketOrderRequest) -> Result<Order<E::OrderIdType>> {
        self.exchange.market_sell(req).await
    }

    pub async fn cancel_order(
        &self,
        req: &CancelOrderRequest<E::OrderIdType>,
    ) -> Result<OrderCanceled<E::OrderIdType>> {
        self.exchange.cancel_order(req).await
    }

    pub async fn cancel_all_orders(
        &self,
        req: &CancelAllOrdersRequest,
    ) -> Result<Vec<OrderCanceled<E::OrderIdType>>> {
        self.exchange.cancel_all_orders(req).await
    }

    pub async fn get_all_open_orders(&self) -> Result<Vec<Order<E::OrderIdType>>> {
        self.exchange.get_all_open_orders().await
    }

    pub async fn get_order_history(
        &self,
        req: &GetOrderHistoryRequest<E::PaginationType>,
    ) -> Result<Vec<Order<E::OrderIdType>>> {
        self.exchange.get_order_history(req).await
    }

    pub async fn get_account_balances(
        &self,
        paginator: Option<&Paginator<E::PaginationType>>,
    ) -> Result<Vec<Balance>> {
        self.exchange.get_account_balances(paginator).await
    }

    pub async fn get_trade_history(
        &self,
        req: &TradeHistoryRequest<E::OrderIdType, E::PaginationType>,
    ) -> Result<Vec<Trade<E::TradeIdType, E::OrderIdType>>> {
        self.exchange.get_trade_history(req).await
    }

    pub async fn get_historic_rates(
        &self,
        req: &GetHistoricRatesRequest<E::PaginationType>,
    ) -> Result<Vec<Candle>> {
        self.exchange.get_historic_rates(req).await
    }

    pub async fn get_historic_trades(
        &self,
        req: &GetHistoricTradesRequest<E::PaginationType>,
    ) -> Result<Vec<Trade<E::TradeIdType, E::OrderIdType>>> {
        self.exchange.get_historic_trades(req).await
    }

    pub async fn get_order(
        &self,
        req: GetOrderRequest<E::OrderIdType>,
    ) -> Result<Order<E::OrderIdType>> {
        self.exchange.get_order(&req).await
    }

    pub async fn get_price_ticker(&self, req: &GetPriceTickerRequest) -> Result<Ticker> {
        self.exchange.get_price_ticker(req).await
    }
}

#[async_trait]
pub trait Exchange {
    type OrderIdType;
    type TradeIdType;
    type PaginationType;
    async fn refresh_market_info(&self) -> Result<Vec<MarketPairHandle>>;
    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse>;
    async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self::OrderIdType>>;
    async fn limit_sell(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self::OrderIdType>>;
    async fn market_buy(&self, req: &OpenMarketOrderRequest) -> Result<Order<Self::OrderIdType>>;
    async fn market_sell(&self, req: &OpenMarketOrderRequest) -> Result<Order<Self::OrderIdType>>;
    async fn cancel_order(
        &self,
        req: &CancelOrderRequest<Self::OrderIdType>,
    ) -> Result<OrderCanceled<Self::OrderIdType>>;
    async fn cancel_all_orders(
        &self,
        req: &CancelAllOrdersRequest,
    ) -> Result<Vec<OrderCanceled<Self::OrderIdType>>>;
    async fn get_all_open_orders(&self) -> Result<Vec<Order<Self::OrderIdType>>>;
    async fn get_order_history(
        &self,
        req: &GetOrderHistoryRequest<Self::PaginationType>,
    ) -> Result<Vec<Order<Self::OrderIdType>>>;
    async fn get_account_balances(
        &self,
        paginator: Option<&Paginator<Self::PaginationType>>,
    ) -> Result<Vec<Balance>>;
    async fn get_trade_history(
        &self,
        req: &TradeHistoryRequest<Self::OrderIdType, Self::PaginationType>,
    ) -> Result<Vec<Trade<Self::TradeIdType, Self::OrderIdType>>>;
    async fn get_price_ticker(&self, req: &GetPriceTickerRequest) -> Result<Ticker>;
    async fn get_historic_rates(
        &self,
        req: &GetHistoricRatesRequest<Self::PaginationType>,
    ) -> Result<Vec<Candle>>;
    async fn get_historic_trades(
        &self,
        req: &GetHistoricTradesRequest<Self::PaginationType>,
    ) -> Result<Vec<Trade<Self::TradeIdType, Self::OrderIdType>>>;
    async fn get_order(
        &self,
        req: &GetOrderRequest<Self::OrderIdType>,
    ) -> Result<Order<Self::OrderIdType>>;
}
