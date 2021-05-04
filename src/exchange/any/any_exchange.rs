use async_trait::async_trait;
use crate::exchange::coinbase::Coinbase;
use crate::exchange::traits::info::ExchangeInfoRetrieval;
use crate::exchange::traits::info::MarketPair;
use crate::exchange::traits::info::MarketPairHandle;
use crate::exchange::traits::Exchange;
use crate::exchange::traits::ExchangeAccount;
use crate::exchange::traits::ExchangeMarketData;
use crate::exchange::nash::Nash;
use crate::exchange::binance::Binance;
use crate::model::{
    Balance, CancelAllOrdersRequest, CancelOrderRequest, Candle,
    GetHistoricRatesRequest, GetHistoricTradesRequest, GetOrderHistoryRequest, GetOrderRequest,
    GetPriceTickerRequest, OpenLimitOrderRequest, OpenMarketOrderRequest, Order,
    OrderBookRequest, OrderBookResponse, OrderCanceled, Paginator, Ticker, Trade,
    TradeHistoryRequest,
};
use super::shared::Result;
use super::InitAnyExchange;


/// Exchange types
pub enum AnyExchange {
    Nash(Nash),
    Binance(Binance),
    Coinbase(Coinbase),
}

#[async_trait]
impl Exchange for AnyExchange {
    type InitParams = InitAnyExchange;
    type InnerClient = ();
    async fn new(params: InitAnyExchange) -> Result<Self> {
        match params {
            InitAnyExchange::Nash(params) => {
                Nash::new(params).await.map(|exchange| exchange.into())
            }
            InitAnyExchange::Binance(params) => {
                Binance::new(params).await.map(|exchange| exchange.into())
            }
            InitAnyExchange::Coinbase(params) => {
                Coinbase::new(params).await.map(|exchange| exchange.into())
            }
        }
    }
    /// not particularly useful to access the inner client with this type. could wrap the inner
    /// client reference in an enum, but that would introduce lifetimes all the way down due to
    /// https://users.rust-lang.org/t/how-to-specify-lifetime-for-associated-type/5736
    fn inner_client(&self) -> Option<&Self::InnerClient> {
        None
    }
}

#[async_trait]
impl ExchangeInfoRetrieval for AnyExchange {
    async fn get_pair(&self, name: &str) -> Result<MarketPairHandle> {
        match self {
            Self::Nash(nash) => nash.get_pair(name).await,
            Self::Binance(binance) => binance.get_pair(name).await,
            Self::Coinbase(coinbase) => coinbase.get_pair(name).await,
        }
    }
    async fn retrieve_pairs(&self) -> Result<Vec<MarketPair>> {
        match self {
            Self::Nash(nash) => nash.retrieve_pairs().await,
            Self::Binance(binance) => binance.retrieve_pairs().await,
            Self::Coinbase(coinbase) => coinbase.retrieve_pairs().await,
        }
    }
    async fn refresh_market_info(&self) -> Result<Vec<MarketPairHandle>> {
        match self {
            Self::Nash(nash) => nash.refresh_market_info().await,
            Self::Binance(binance) => binance.refresh_market_info().await,
            Self::Coinbase(coinbase) => coinbase.refresh_market_info().await,
        }
    }
}

#[async_trait]
impl ExchangeAccount for AnyExchange {
    async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order> {
        match self {
            Self::Nash(nash) => nash.limit_buy(req).await,
            Self::Binance(binance) => binance.limit_buy(req).await,
            Self::Coinbase(coinbase) => coinbase.limit_buy(req).await,
        }
    }
    async fn limit_sell(&self, req: &OpenLimitOrderRequest) -> Result<Order> {
        match self {
            Self::Nash(nash) => nash.limit_sell(req).await,
            Self::Binance(binance) => binance.limit_sell(req).await,
            Self::Coinbase(coinbase) => coinbase.limit_sell(req).await,
        }
    }
    async fn market_buy(&self, req: &OpenMarketOrderRequest) -> Result<Order> {
        match self {
            Self::Nash(nash) => nash.market_buy(req).await,
            Self::Binance(binance) => binance.market_buy(req).await,
            Self::Coinbase(coinbase) => coinbase.market_buy(req).await,
        }
    }
    async fn market_sell(&self, req: &OpenMarketOrderRequest) -> Result<Order> {
        match self {
            Self::Nash(nash) => nash.market_sell(req).await,
            Self::Binance(binance) => binance.market_sell(req).await,
            Self::Coinbase(coinbase) => coinbase.market_sell(req).await,
        }
    }
    async fn cancel_order(&self, req: &CancelOrderRequest) -> Result<OrderCanceled> {
        match self {
            Self::Nash(nash) => nash.cancel_order(req).await,
            Self::Binance(binance) => binance.cancel_order(req).await,
            Self::Coinbase(coinbase) => coinbase.cancel_order(req).await,
        }
    }
    async fn cancel_all_orders(&self, req: &CancelAllOrdersRequest) -> Result<Vec<OrderCanceled>> {
        match self {
            Self::Nash(nash) => nash.cancel_all_orders(req).await,
            Self::Binance(binance) => binance.cancel_all_orders(req).await,
            Self::Coinbase(coinbase) => coinbase.cancel_all_orders(req).await,
        }
    }
    async fn get_all_open_orders(&self) -> Result<Vec<Order>> {
        match self {
            Self::Nash(nash) => nash.get_all_open_orders().await,
            Self::Binance(binance) => binance.get_all_open_orders().await,
            Self::Coinbase(coinbase) => coinbase.get_all_open_orders().await,
        }
    }
    async fn get_order_history(&self, req: &GetOrderHistoryRequest) -> Result<Vec<Order>> {
        match self {
            Self::Nash(nash) => nash.get_order_history(req).await,
            Self::Binance(binance) => binance.get_order_history(req).await,
            Self::Coinbase(coinbase) => coinbase.get_order_history(req).await,
        }
    }
    async fn get_trade_history(&self, req: &TradeHistoryRequest) -> Result<Vec<Trade>> {
        match self {
            Self::Nash(nash) => nash.get_trade_history(req).await,
            Self::Binance(binance) => binance.get_trade_history(req).await,
            Self::Coinbase(coinbase) => coinbase.get_trade_history(req).await,
        }
    }
    async fn get_account_balances(&self, paginator: Option<Paginator>) -> Result<Vec<Balance>> {
        match self {
            Self::Nash(nash) => nash.get_account_balances(paginator).await,
            Self::Binance(binance) => binance.get_account_balances(paginator).await,
            Self::Coinbase(coinbase) => coinbase.get_account_balances(paginator).await,
        }
    }
    async fn get_order(&self, req: &GetOrderRequest) -> Result<Order> {
        match self {
            Self::Nash(nash) => nash.get_order(req).await,
            Self::Binance(binance) => binance.get_order(req).await,
            Self::Coinbase(coinbase) => coinbase.get_order(req).await,
        }
    }
}

#[async_trait]
impl ExchangeMarketData for AnyExchange {
    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse> {
        match self {
            Self::Nash(nash) => nash.order_book(req).await,
            Self::Binance(binance) => binance.order_book(req).await,
            Self::Coinbase(coinbase) => coinbase.order_book(req).await,
        }
    }
    async fn get_price_ticker(&self, req: &GetPriceTickerRequest) -> Result<Ticker> {
        match self {
            Self::Nash(nash) => nash.get_price_ticker(req).await,
            Self::Binance(binance) => binance.get_price_ticker(req).await,
            Self::Coinbase(coinbase) => coinbase.get_price_ticker(req).await,
        }
    }
    async fn get_historic_rates(&self, req: &GetHistoricRatesRequest) -> Result<Vec<Candle>> {
        match self {
            Self::Nash(nash) => nash.get_historic_rates(req).await,
            Self::Binance(binance) => binance.get_historic_rates(req).await,
            Self::Coinbase(coinbase) => coinbase.get_historic_rates(req).await,
        }
    }
    async fn get_historic_trades(&self, req: &GetHistoricTradesRequest) -> Result<Vec<Trade>> {
        match self {
            Self::Nash(nash) => nash.get_historic_trades(req).await,
            Self::Binance(binance) => binance.get_historic_trades(req).await,
            Self::Coinbase(coinbase) => coinbase.get_historic_trades(req).await,
        }
    }
}

impl From<Coinbase> for AnyExchange {
    fn from(coinbase: Coinbase) -> Self {
        Self::Coinbase(coinbase)
    }
}

impl From<Nash> for AnyExchange {
    fn from(nash: Nash) -> Self {
        Self::Nash(nash)
    }
}

impl From<Binance> for AnyExchange {
    fn from(binance: Binance) -> Self {
        Self::Binance(binance)
    }
}