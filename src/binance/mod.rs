pub mod client;
pub mod model;
mod transport;

use crate::{
    errors::OpenLimitError,
    exchange::Exchange,
    exchange_info::ExchangeInfo,
    model::{
        AskBid, Balance, CancelAllOrdersRequest, CancelOrderRequest, Candle,
        GetHistoricRatesRequest, GetOrderHistoryRequest, GetOrderRequest, GetPriceTickerRequest,
        Interval, Liquidity, OpenLimitOrderRequest, OpenMarketOrderRequest, Order,
        OrderBookRequest, OrderBookResponse, OrderCanceled, OrderStatus, Paginator, Side, Ticker,
        Trade, TradeHistoryRequest, Transaction,
    },
    shared::Result,
};
use async_trait::async_trait;
use model::KlineSummaries;
use transport::Transport;

#[derive(Clone)]
pub struct Binance {
    exchange_info: ExchangeInfo,
    transport: Transport,
}

impl Binance {
    pub async fn new(sandbox: bool) -> Self {
        Binance {
            exchange_info: ExchangeInfo::new(),
            transport: Transport::new(sandbox).unwrap(),
        }
    }

    pub async fn with_credential(api_key: &str, api_secret: &str, sandbox: bool) -> Self {
        Binance {
            exchange_info: ExchangeInfo::new(),
            transport: Transport::with_credential(api_key, api_secret, sandbox).unwrap(),
        }
    }
}

#[async_trait]
impl Exchange for Binance {
    type OrderIdType = u64;
    type TradeIdType = u64;
    type PaginationType = u64;

    async fn order_book(&mut self, req: &OrderBookRequest) -> Result<OrderBookResponse> {
        self.get_depth(req.market_pair.as_str(), None)
            .await
            .map(Into::into)
    }

    async fn limit_buy(&mut self, req: &OpenLimitOrderRequest) -> Result<Order<Self::OrderIdType>> {
        Binance::limit_buy(self, &req.market_pair, req.size, req.price)
            .await
            .map(Into::into)
    }
    async fn limit_sell(
        &mut self,
        req: &OpenLimitOrderRequest,
    ) -> Result<Order<Self::OrderIdType>> {
        Binance::limit_sell(self, &req.market_pair, req.size, req.price)
            .await
            .map(Into::into)
    }

    async fn market_buy(
        &mut self,
        req: &OpenMarketOrderRequest,
    ) -> Result<Order<Self::OrderIdType>> {
        Binance::market_buy(self, &req.market_pair, req.size)
            .await
            .map(Into::into)
    }
    async fn market_sell(
        &mut self,
        req: &OpenMarketOrderRequest,
    ) -> Result<Order<Self::OrderIdType>> {
        Binance::market_sell(self, &req.market_pair, req.size)
            .await
            .map(Into::into)
    }
    async fn cancel_order(
        &mut self,
        req: &CancelOrderRequest<Self::OrderIdType>,
    ) -> Result<OrderCanceled<Self::OrderIdType>> {
        if let Some(pair) = req.market_pair.as_ref() {
            Binance::cancel_order(self, pair.as_ref(), req.id)
                .await
                .map(Into::into)
        } else {
            Err(OpenLimitError::MissingParameter(
                "pair parameter is required.".to_string(),
            ))
        }
    }
    async fn cancel_all_orders(
        &mut self,
        req: &CancelAllOrdersRequest,
    ) -> Result<Vec<OrderCanceled<Self::OrderIdType>>> {
        if let Some(pair) = req.market_pair.as_ref() {
            Binance::cancel_all_orders(self, pair.as_ref())
                .await
                .map(|v| v.into_iter().map(Into::into).collect())
        } else {
            Err(OpenLimitError::MissingParameter(
                "pair parameter is required.".to_string(),
            ))
        }
    }
    async fn get_all_open_orders(&mut self) -> Result<Vec<Order<Self::OrderIdType>>> {
        Binance::get_all_open_orders(self)
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_order_history(
        &mut self,
        req: &GetOrderHistoryRequest<Self::PaginationType>,
    ) -> Result<Vec<Order<Self::OrderIdType>>> {
        let req = req.into();
        Binance::get_all_orders(self, &req)
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_account_balances(
        &mut self,
        _paginator: Option<&Paginator<Self::PaginationType>>,
    ) -> Result<Vec<Balance>> {
        Binance::get_account(self)
            .await
            .map(|v| v.balances.into_iter().map(Into::into).collect())
    }

    async fn get_trade_history(
        &mut self,
        req: &TradeHistoryRequest<Self::OrderIdType, Self::PaginationType>,
    ) -> Result<Vec<Trade<Self::TradeIdType, Self::OrderIdType>>> {
        let req = req.into();
        Binance::trade_history(self, &req)
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_price_ticker(&mut self, req: &GetPriceTickerRequest) -> Result<Ticker> {
        Binance::get_price(self, &req.market_pair)
            .await
            .map(Into::into)
    }

    async fn get_historic_rates(
        &mut self,
        req: &GetHistoricRatesRequest<Self::PaginationType>,
    ) -> Result<Vec<Candle>> {
        let params = req.into();

        Binance::get_klines(self, &params)
            .await
            .map(|KlineSummaries::AllKlineSummaries(v)| v.into_iter().map(Into::into).collect())
    }

    async fn get_order(
        &mut self,
        req: &GetOrderRequest<Self::OrderIdType>,
    ) -> Result<Order<Self::OrderIdType>> {
        let pair = req.market_pair.clone().unwrap();
        Binance::get_order(self, &pair, req.id)
            .await
            .map(Into::into)
    }

    async fn refresh_market_info(&mut self) -> Result<()> {
        self.exchange_info.refresh(self).await
    }
}

impl From<model::OrderBook> for OrderBookResponse {
    fn from(book: model::OrderBook) -> Self {
        Self {
            last_update_id: Some(book.last_update_id),
            bids: book.bids.into_iter().map(Into::into).collect(),
            asks: book.asks.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<model::AskBid> for AskBid {
    fn from(bids: model::AskBid) -> Self {
        Self {
            price: bids.price,
            qty: bids.qty,
        }
    }
}

impl From<model::Transaction> for Transaction<u64> {
    fn from(order: model::Transaction) -> Self {
        Self {
            id: order.order_id,
            market_pair: order.symbol,
            client_order_id: Some(order.client_order_id),
            created_at: order.transact_time,
        }
    }
}

impl From<model::Order> for Order<u64> {
    fn from(order: model::Order) -> Self {
        Self {
            id: order.order_id,
            market_pair: order.symbol,
            client_order_id: Some(order.client_order_id),
            created_at: order.time,
            order_type: order.type_name,
            side: order.side.into(),
            status: order.status.into(),
            price: Some(order.price),
            size: order.orig_qty,
        }
    }
}

impl From<model::OrderCanceled> for OrderCanceled<u64> {
    fn from(order: model::OrderCanceled) -> Self {
        Self { id: order.order_id }
    }
}

impl From<model::Balance> for Balance {
    fn from(balance: model::Balance) -> Self {
        Self {
            asset: balance.asset,
            free: balance.free,
            total: balance.locked + balance.free,
        }
    }
}

impl From<model::TradeHistory> for Trade<u64, u64> {
    fn from(trade_history: model::TradeHistory) -> Self {
        Self {
            id: trade_history.id,
            order_id: trade_history.order_id,
            market_pair: trade_history.symbol,
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

impl From<model::SymbolPrice> for Ticker {
    fn from(ticker: model::SymbolPrice) -> Self {
        Self {
            price: ticker.price,
        }
    }
}

impl From<&GetOrderHistoryRequest<u64>> for model::AllOrderReq {
    fn from(req: &GetOrderHistoryRequest<u64>) -> Self {
        Self {
            paginator: req.paginator.clone().map(|p| p.into()),
            symbol: req.market_pair.clone().unwrap(),
        }
    }
}

impl From<&TradeHistoryRequest<u64, u64>> for model::TradeHistoryReq {
    fn from(trade_history: &TradeHistoryRequest<u64, u64>) -> Self {
        Self {
            paginator: trade_history.paginator.clone().map(|p| p.into()),
            symbol: trade_history.market_pair.clone().unwrap(),
        }
    }
}

impl From<&GetHistoricRatesRequest<u64>> for model::KlineParams {
    fn from(req: &GetHistoricRatesRequest<u64>) -> Self {
        let interval: &str = req.interval.into();

        Self {
            interval: String::from(interval),
            paginator: req.paginator.clone().map(|d| d.into()),
            symbol: req.market_pair.clone(),
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

impl From<model::KlineSummary> for Candle {
    fn from(kline_summary: model::KlineSummary) -> Self {
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

impl From<Paginator<u64>> for model::Paginator {
    fn from(paginator: Paginator<u64>) -> Self {
        Self {
            from_id: paginator.after,
            order_id: paginator.after,
            end_time: paginator.end_time,
            start_time: paginator.start_time,
            limit: paginator.limit,
        }
    }
}

impl From<String> for Side {
    fn from(side: String) -> Self {
        if side == "buy" {
            Side::Buy
        } else {
            Side::Sell
        }
    }
}

impl From<model::OrderStatus> for OrderStatus {
    fn from(status: model::OrderStatus) -> OrderStatus {
        match status {
            model::OrderStatus::Canceled => OrderStatus::Canceled,
            model::OrderStatus::Expired => OrderStatus::Expired,
            model::OrderStatus::Filled => OrderStatus::Filled,
            model::OrderStatus::New => OrderStatus::New,
            model::OrderStatus::PartiallyFilled => OrderStatus::PartiallyFilled,
            model::OrderStatus::PendingCancel => OrderStatus::PendingCancel,
            model::OrderStatus::Rejected => OrderStatus::Rejected,
        }
    }
}
