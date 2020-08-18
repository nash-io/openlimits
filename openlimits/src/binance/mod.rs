use async_trait::async_trait;
use derive_more::{Deref, DerefMut};
use shared::Result;

use crate::exchange::Exchange;
use crate::model::{
    Asks, Balance, Bids, CancelAllOrdersRequest, CancelOrderRequest, Candle,
    GetHistoricRatesRequest, GetOrderHistoryRequest, GetPriceTickerRequest, Interval, Liquidity,
    OpenLimitOrderRequest, OpenMarketOrderRequest, Order, OrderBookRequest, OrderBookResponse,
    OrderCanceled, Paginator, Side, Ticker, Trade, TradeHistoryRequest,
};
use binance::model::KlineSummaries;
use shared::errors::OpenLimitError;

#[derive(Deref, DerefMut)]
pub struct Binance(binance::Binance);

impl Binance {
    pub fn new(sandbox: bool) -> Self {
        Binance(binance::Binance::new(sandbox))
    }

    pub fn with_credential(api_key: &str, api_secret: &str, sandbox: bool) -> Self {
        Binance(binance::Binance::with_credential(
            api_key, api_secret, sandbox,
        ))
    }
}

#[async_trait]
impl Exchange for Binance {
    type OrderIdType = u64;
    type TradeIdType = u64;

    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse> {
        self.get_depth(req.symbol.as_str(), None)
            .await
            .map(Into::into)
    }

    async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self::OrderIdType>> {
        binance::Binance::limit_buy(self, &req.symbol, req.size, req.price)
            .await
            .map(Into::into)
    }
    async fn limit_sell(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self::OrderIdType>> {
        binance::Binance::limit_sell(self, &req.symbol, req.size, req.price)
            .await
            .map(Into::into)
    }

    async fn market_buy(&self, req: &OpenMarketOrderRequest) -> Result<Order<Self::OrderIdType>> {
        binance::Binance::market_buy(self, &req.symbol, req.size)
            .await
            .map(Into::into)
    }
    async fn market_sell(&self, req: &OpenMarketOrderRequest) -> Result<Order<Self::OrderIdType>> {
        binance::Binance::market_sell(self, &req.symbol, req.size)
            .await
            .map(Into::into)
    }
    async fn cancel_order(
        &self,
        req: &CancelOrderRequest<Self::OrderIdType>,
    ) -> Result<OrderCanceled<Self::OrderIdType>> {
        if let Some(pair) = req.pair.as_ref() {
            binance::Binance::cancel_order(self, pair.as_ref(), req.id)
                .await
                .map(Into::into)
        } else {
            Err(OpenLimitError::MissingParameter(
                "pair parameter is required.".to_string(),
            ))
        }
    }
    async fn cancel_all_orders(
        &self,
        req: &CancelAllOrdersRequest,
    ) -> Result<Vec<OrderCanceled<Self::OrderIdType>>> {
        if let Some(pair) = req.pair.as_ref() {
            binance::Binance::cancel_all_orders(self, pair.as_ref())
                .await
                .map(|v| v.into_iter().map(Into::into).collect())
        } else {
            Err(OpenLimitError::MissingParameter(
                "pair parameter is required.".to_string(),
            ))
        }
    }
    async fn get_all_open_orders(&self) -> Result<Vec<Order<Self::OrderIdType>>> {
        binance::Binance::get_all_open_orders(self)
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_order_history(
        &self,
        req: &GetOrderHistoryRequest,
    ) -> Result<Vec<Order<Self::OrderIdType>>> {
        let req = req.into();
        binance::Binance::get_all_orders(self, &req)
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_account_balances(&self, _paginator: Option<&Paginator>) -> Result<Vec<Balance>> {
        binance::Binance::get_account(self)
            .await
            .map(|v| v.balances.into_iter().map(Into::into).collect())
    }

    async fn get_trade_history(
        &self,
        req: &TradeHistoryRequest<Self::OrderIdType>,
    ) -> Result<Vec<Trade<Self::TradeIdType, Self::OrderIdType>>> {
        let req = req.into();
        binance::Binance::trade_history(self, &req)
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_price_ticker(&self, req: &GetPriceTickerRequest) -> Result<Ticker> {
        binance::Binance::get_price(self, &req.symbol)
            .await
            .map(Into::into)
    }

    async fn get_historic_rates(&self, req: &GetHistoricRatesRequest) -> Result<Vec<Candle>> {
        let params = req.into();

        binance::Binance::get_klines(self, &params)
            .await
            .map(|KlineSummaries::AllKlineSummaries(v)| v.into_iter().map(Into::into).collect())
    }
}

impl From<binance::model::OrderBook> for OrderBookResponse {
    fn from(book: binance::model::OrderBook) -> Self {
        Self {
            last_update_id: Some(book.last_update_id),
            bids: book.bids.into_iter().map(Into::into).collect(),
            asks: book.asks.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<binance::model::Bids> for Bids {
    fn from(bids: binance::model::Bids) -> Self {
        Self {
            price: bids.price,
            qty: bids.qty,
        }
    }
}

impl From<binance::model::Asks> for Asks {
    fn from(bids: binance::model::Asks) -> Self {
        Self {
            price: bids.price,
            qty: bids.qty,
        }
    }
}

impl From<binance::model::Transaction> for Order<u64> {
    fn from(order: binance::model::Transaction) -> Self {
        Self {
            id: order.order_id,
            symbol: order.symbol,
            client_order_id: Some(order.client_order_id),
            created_at: order.transact_time,
        }
    }
}

impl From<binance::model::Order> for Order<u64> {
    fn from(order: binance::model::Order) -> Self {
        Self {
            id: order.order_id,
            symbol: order.symbol,
            client_order_id: Some(order.client_order_id),
            created_at: order.time,
        }
    }
}

impl From<binance::model::OrderCanceled> for OrderCanceled<u64> {
    fn from(order: binance::model::OrderCanceled) -> Self {
        Self { id: order.order_id }
    }
}

impl From<binance::model::Balance> for Balance {
    fn from(balance: binance::model::Balance) -> Self {
        Self {
            asset: balance.asset,
            free: balance.free,
            total: balance.locked + balance.free,
        }
    }
}

impl From<binance::model::TradeHistory> for Trade<u64, u64> {
    fn from(trade_history: binance::model::TradeHistory) -> Self {
        Self {
            id: trade_history.id,
            order_id: trade_history.order_id,
            pair: trade_history.symbol,
            price: trade_history.price,
            qty: trade_history.qty,
            fees: trade_history.commission,
            side: match trade_history.is_buyer {
                true => Side::Buy,
                false => Side::Sell,
            },
            liquidity: match trade_history.is_maker {
                true => Some(Liquidity::Maker),
                false => Some(Liquidity::Taker),
            },
            created_at: trade_history.time,
        }
    }
}

impl From<binance::model::SymbolPrice> for Ticker {
    fn from(ticker: binance::model::SymbolPrice) -> Self {
        Self {
            price: ticker.price,
        }
    }
}

impl From<&GetOrderHistoryRequest> for binance::model::AllOrderReq {
    fn from(req: &GetOrderHistoryRequest) -> Self {
        Self {
            paginator: req.paginator.clone().map(|p| p.into()),
            symbol: req.symbol.clone().unwrap(),
        }
    }
}

impl From<&TradeHistoryRequest<u64>> for binance::model::TradeHistoryReq {
    fn from(trade_history: &TradeHistoryRequest<u64>) -> Self {
        Self {
            paginator: trade_history.paginator.clone().map(|p| p.into()),
            symbol: trade_history.pair.clone().unwrap(),
        }
    }
}

impl From<&GetHistoricRatesRequest> for binance::model::KlineParams {
    fn from(req: &GetHistoricRatesRequest) -> Self {
        let interval: &str = req.interval.into();

        Self {
            interval: String::from(interval),
            paginator: req.paginator.clone().map(|d| d.into()),
            symbol: req.symbol.clone(),
        }
    }
}

impl From<Interval> for &str {
    fn from(interval: Interval) -> Self {
        match interval {
            Interval::OneMinute => "1m",
            Interval::ThreeMinutes => "3m",
            Interval::FiveMinutes => "5m",
            Interval::FiftyMinutes => "15m",
            Interval::ThirtyMinutes => "30m",
            Interval::OneHour => "1h",
            Interval::TwoHours => "2h",
            Interval::FourHours => "4h",
            Interval::SixHours => "6h",
            Interval::EightHours => "8h",
            Interval::TwelveHours => "12h",
            Interval::OneDay => "1d",
            Interval::ThreeDay => "3d",
            Interval::OneWeek => "1w",
            Interval::OneMonth => "1M",
        }
    }
}

impl From<binance::model::KlineSummary> for Candle {
    fn from(kline_summary: binance::model::KlineSummary) -> Self {
        Self {
            time: kline_summary.open_time as u64,
            low: kline_summary.low,
            high: kline_summary.high,
            open: kline_summary.open,
            close: kline_summary.close,
            volume: kline_summary.volume,
        }
    }
}

impl From<Paginator> for binance::model::Paginator {
    fn from(paginator: Paginator) -> Self {
        Self {
            from_id: paginator.after,
            order_id: paginator.after,
            end_time: paginator.end_time,
            start_time: paginator.start_time,
            limit: paginator.limit,
        }
    }
}
