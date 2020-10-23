pub mod client;
pub mod model;
mod transport;
use std::convert::TryFrom;

use crate::exchange_info::ExchangeInfoRetrieval;
use crate::{
    binance::model::websocket::TradeMessage,
    errors::OpenLimitError,
    exchange::ExchangeAccount,
    exchange::{Exchange, ExchangeEssentials, ExchangeMarketData, ExchangeSpec},
    exchange_info::ExchangeInfo,
    exchange_ws::ExchangeWs,
    model::{
        websocket::{OpenLimitsWebsocketMessage, Subscription},
        AskBid, Balance, CancelAllOrdersRequest, CancelOrderRequest, Candle,
        GetHistoricRatesRequest, GetHistoricTradesRequest, GetOrderHistoryRequest, GetOrderRequest,
        GetPriceTickerRequest, Interval, Liquidity, OpenLimitOrderRequest, OpenMarketOrderRequest,
        Order, OrderBookRequest, OrderBookResponse, OrderCanceled, OrderStatus, Paginator, Side,
        Ticker, Trade, TradeHistoryRequest, Transaction,
    },
    shared::Result,
};
use async_trait::async_trait;
use client::websocket::BinanceWebsocket;
use model::KlineSummaries;
use transport::Transport;

#[derive(Clone)]
pub struct Binance {
    exchange_info: ExchangeInfo,
    transport: Transport,
}

pub struct BinanceCredentials {
    pub api_key: String,
    pub api_secret: String,
}

#[derive(Default)]
pub struct BinanceParameters {
    pub sandbox: bool,
    pub credentials: Option<BinanceCredentials>,
}

impl BinanceParameters {
    pub fn sandbox() -> Self {
        Self {
            sandbox: true,
            ..Default::default()
        }
    }

    pub fn prod() -> Self {
        Self {
            sandbox: false,
            ..Default::default()
        }
    }
}

#[async_trait]
impl ExchangeEssentials for Binance {
    type Parameters = BinanceParameters;

    async fn new(parameters: Self::Parameters) -> Self {
        let binance = match parameters.credentials {
            Some(credentials) => Binance {
                exchange_info: ExchangeInfo::new(),
                transport: Transport::with_credential(
                    &credentials.api_key,
                    &credentials.api_secret,
                    parameters.sandbox,
                )
                .unwrap(),
            },
            None => Binance {
                exchange_info: ExchangeInfo::new(),
                transport: Transport::new(parameters.sandbox).unwrap(),
            },
        };

        binance.refresh_market_info().await.unwrap();
        binance
    }
}

#[async_trait]
impl ExchangeSpec for Exchange<Binance> {
    type OrderId = u64;
    type TradeId = u64;
    type Pagination = u64;
}

#[async_trait]
impl ExchangeMarketData for Exchange<Binance> {
    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse> {
        self.inner
            .get_depth(req.market_pair.as_str(), None)
            .await
            .map(Into::into)
    }

    async fn get_price_ticker(&self, req: &GetPriceTickerRequest) -> Result<Ticker> {
        Binance::get_price(&self.inner, &req.market_pair)
            .await
            .map(Into::into)
    }

    async fn get_historic_rates(&self, req: &GetHistoricRatesRequest<Self>) -> Result<Vec<Candle>> {
        let params = req.into();

        Binance::get_klines(&self.inner, &params)
            .await
            .map(|KlineSummaries::AllKlineSummaries(v)| v.into_iter().map(Into::into).collect())
    }

    async fn get_historic_trades(
        &self,
        _req: &GetHistoricTradesRequest<Self>,
    ) -> Result<Vec<Trade<Self>>> {
        unimplemented!("Only implemented for Nash right now");
    }
}

#[async_trait]
impl ExchangeAccount for Exchange<Binance> {
    async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self>> {
        Binance::limit_buy(&self.inner, &req.market_pair, req.size, req.price)
            .await
            .map(Into::into)
    }
    async fn limit_sell(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self>> {
        Binance::limit_sell(&self.inner, &req.market_pair, req.size, req.price)
            .await
            .map(Into::into)
    }

    async fn market_buy(&self, req: &OpenMarketOrderRequest) -> Result<Order<Self>> {
        Binance::market_buy(&self.inner, &req.market_pair, req.size)
            .await
            .map(Into::into)
    }
    async fn market_sell(&self, req: &OpenMarketOrderRequest) -> Result<Order<Self>> {
        Binance::market_sell(&self.inner, &req.market_pair, req.size)
            .await
            .map(Into::into)
    }
    async fn cancel_order(&self, req: &CancelOrderRequest<Self>) -> Result<OrderCanceled<Self>> {
        if let Some(pair) = req.market_pair.as_ref() {
            Binance::cancel_order(&self.inner, pair.as_ref(), req.id)
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
    ) -> Result<Vec<OrderCanceled<Self>>> {
        if let Some(pair) = req.market_pair.as_ref() {
            Binance::cancel_all_orders(&self.inner, pair.as_ref())
                .await
                .map(|v| v.into_iter().map(Into::into).collect())
        } else {
            Err(OpenLimitError::MissingParameter(
                "pair parameter is required.".to_string(),
            ))
        }
    }
    async fn get_all_open_orders(&self) -> Result<Vec<Order<Self>>> {
        Binance::get_all_open_orders(&self.inner)
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_order_history(
        &self,
        req: &GetOrderHistoryRequest<Self>,
    ) -> Result<Vec<Order<Self>>> {
        let req = model::AllOrderReq::try_from(req)?;
        Binance::get_all_orders(&self.inner, &req)
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_trade_history(&self, req: &TradeHistoryRequest<Self>) -> Result<Vec<Trade<Self>>> {
        let req = model::TradeHistoryReq::try_from(req)?;
        Binance::trade_history(&self.inner, &req)
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_account_balances(
        &self,
        _paginator: Option<&Paginator<Self>>,
    ) -> Result<Vec<Balance>> {
        Binance::get_account(&self.inner)
            .await
            .map(|v| v.balances.into_iter().map(Into::into).collect())
    }

    async fn get_order(&self, req: &GetOrderRequest<Self>) -> Result<Order<Self>> {
        let pair = req.market_pair.clone().ok_or_else(|| {
            OpenLimitError::MissingParameter("market_pair parameter is required.".to_string())
        })?;
        Binance::get_order(&self.inner, &pair, req.id)
            .await
            .map(Into::into)
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

impl From<TradeMessage> for Trade<Exchange<Binance>> {
    fn from(trade: TradeMessage) -> Self {
        Self {
            id: trade.trade_id,
            order_id: trade.buyer_order_id,
            market_pair: trade.symbol,
            price: trade.price,
            qty: trade.qty,
            fees: None, // Binance does not return fee on trades over WS stream
            // https://money.stackexchange.com/questions/90686/what-does-buyer-is-maker-mean/102005#102005
            side: match trade.is_buyer_maker {
                true => Side::Sell,
                false => Side::Buy,
            },
            liquidity: None,
            created_at: trade.trade_order_time,
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

impl From<model::Order> for Order<Exchange<Binance>> {
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

impl From<model::OrderCanceled> for OrderCanceled<Exchange<Binance>> {
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

impl From<model::TradeHistory> for Trade<Exchange<Binance>> {
    fn from(trade_history: model::TradeHistory) -> Self {
        Self {
            id: trade_history.id,
            order_id: trade_history.order_id,
            market_pair: trade_history.symbol,
            price: trade_history.price,
            qty: trade_history.qty,
            fees: Some(trade_history.commission),
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

impl TryFrom<&GetOrderHistoryRequest<Exchange<Binance>>> for model::AllOrderReq {
    type Error = OpenLimitError;
    fn try_from(req: &GetOrderHistoryRequest<Exchange<Binance>>) -> Result<Self> {
        Ok(Self {
            paginator: req.paginator.clone().map(|p| p.into()),
            symbol: req.market_pair.clone().ok_or_else(|| {
                OpenLimitError::MissingParameter("market_pair parameter is required.".to_string())
            })?,
        })
    }
}

impl TryFrom<&TradeHistoryRequest<Exchange<Binance>>> for model::TradeHistoryReq {
    type Error = OpenLimitError;
    fn try_from(trade_history: &TradeHistoryRequest<Exchange<Binance>>) -> Result<Self> {
        Ok(Self {
            paginator: trade_history.paginator.clone().map(|p| p.into()),
            symbol: trade_history.market_pair.clone().ok_or_else(|| {
                OpenLimitError::MissingParameter("market_pair parameter is required.".to_string())
            })?,
        })
    }
}

impl From<&GetHistoricRatesRequest<Exchange<Binance>>> for model::KlineParams {
    fn from(req: &GetHistoricRatesRequest<Exchange<Binance>>) -> Self {
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
            Interval::FifteenMinutes => "15m",
            Interval::ThirtyMinutes => "30m",
            Interval::OneHour => "1h",
            Interval::TwoHours => "2h",
            Interval::FourHours => "4h",
            Interval::SixHours => "6h",
            Interval::EightHours => "8h",
            Interval::TwelveHours => "12h",
            Interval::OneDay => "1d",
            Interval::ThreeDays => "3d",
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

impl From<Paginator<Exchange<Binance>>> for model::Paginator {
    fn from(paginator: Paginator<Exchange<Binance>>) -> Self {
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

#[async_trait]
impl ExchangeWs<Exchange<Binance>> for BinanceWebsocket {
    async fn subscribe(&mut self, subscription: Subscription) -> Result<()> {
        BinanceWebsocket::subscribe(self, subscription.into()).await
    }
    fn parse_message(
        &self,
        message: Self::Item,
    ) -> Result<OpenLimitsWebsocketMessage<Exchange<Binance>>> {
        Ok(message?.into())
    }
}
impl From<Subscription> for model::websocket::Subscription {
    fn from(sub: Subscription) -> Self {
        match sub {
            Subscription::OrderBook(symbol, depth) => {
                model::websocket::Subscription::OrderBook(symbol, depth)
            }
            Subscription::Trade(symbol) => model::websocket::Subscription::Trade(symbol),
            _ => panic!("Not supported Subscription"),
        }
    }
}

impl From<model::websocket::BinanceWebsocketMessage>
    for OpenLimitsWebsocketMessage<Exchange<Binance>>
{
    fn from(message: model::websocket::BinanceWebsocketMessage) -> Self {
        match message {
            model::websocket::BinanceWebsocketMessage::Ping => OpenLimitsWebsocketMessage::Ping,
            model::websocket::BinanceWebsocketMessage::Trade(trade) => {
                OpenLimitsWebsocketMessage::Trades(vec![trade.into()])
            }
            model::websocket::BinanceWebsocketMessage::OrderBook(orderbook) => {
                OpenLimitsWebsocketMessage::OrderBook(orderbook.into())
            }
            _ => panic!("Not supported Message"),
        }
    }
}
