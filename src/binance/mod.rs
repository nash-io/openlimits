pub mod client;
pub mod model;
mod transport;
use std::convert::TryFrom;

use crate::{
    binance::model::{websocket::TradeMessage, SymbolFilter, ORDER_TYPE_LIMIT, ORDER_TYPE_MARKET},
    errors::OpenLimitError,
    exchange::ExchangeAccount,
    exchange::{Exchange, ExchangeMarketData},
    exchange_info::{ExchangeInfo, ExchangeInfoRetrieval, MarketPair, MarketPairHandle},
    model::{
        AskBid, Balance, CancelAllOrdersRequest, CancelOrderRequest, Candle,
        GetHistoricRatesRequest, GetHistoricTradesRequest, GetOrderHistoryRequest, GetOrderRequest,
        GetPriceTickerRequest, Interval, Liquidity, OpenLimitOrderRequest, OpenMarketOrderRequest,
        Order, OrderBookRequest, OrderBookResponse, OrderCanceled, OrderStatus, OrderType,
        Paginator, Side, Ticker, TimeInForce, Trade, TradeHistoryRequest, Transaction,
    },
    shared::Result,
};
use async_trait::async_trait;
pub use client::websocket::BinanceWebsocket;
use model::KlineSummaries;
use transport::Transport;

use client::BaseClient;

#[derive(Clone)]
pub struct Binance {
    exchange_info: ExchangeInfo,
    client: BaseClient,
}

#[derive(Clone)]
pub struct BinanceCredentials {
    pub api_key: String,
    pub api_secret: String,
}

#[derive(Default, Clone)]
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
impl Exchange for Binance {
    type InitParams = BinanceParameters;
    type InnerClient = BaseClient;

    async fn new(parameters: Self::InitParams) -> Self {
        let binance = match parameters.credentials {
            Some(credentials) => Binance {
                exchange_info: ExchangeInfo::new(),
                client: BaseClient {
                    transport: Transport::with_credential(
                        &credentials.api_key,
                        &credentials.api_secret,
                        parameters.sandbox,
                    )
                    .unwrap(),
                },
            },
            None => Binance {
                exchange_info: ExchangeInfo::new(),
                client: BaseClient {
                    transport: Transport::new(parameters.sandbox).unwrap(),
                },
            },
        };

        binance.refresh_market_info().await.unwrap();
        binance
    }

    fn inner_client(&self) -> Option<&Self::InnerClient> {
        Some(&self.client)
    }
}

#[async_trait]
impl ExchangeInfoRetrieval for Binance {
    async fn retrieve_pairs(&self) -> Result<Vec<MarketPair>> {
        self.client.get_exchange_info().await.map(|v| {
            v.symbols
                .into_iter()
                .map(|symbol| {
                    let lot_size = symbol
                        .filters
                        .iter()
                        .find_map(|f| match f {
                            SymbolFilter::LotSize {
                                max_qty: _,
                                min_qty: _,
                                step_size,
                            } => Some(step_size),
                            _ => None,
                        })
                        .unwrap();

                    let tick_size = symbol
                        .filters
                        .iter()
                        .find_map(|f| match f {
                            SymbolFilter::PriceFilter {
                                min_price: _,
                                max_price: _,
                                tick_size,
                            } => Some(tick_size),
                            _ => None,
                        })
                        .unwrap();

                    MarketPair {
                        base: symbol.base_asset,
                        quote: symbol.quote_asset,
                        symbol: symbol.symbol,
                        base_increment: *lot_size,
                        quote_increment: *tick_size,
                    }
                })
                .collect()
        })
    }

    async fn refresh_market_info(&self) -> Result<Vec<MarketPairHandle>> {
        self.exchange_info
            .refresh(self as &dyn ExchangeInfoRetrieval)
            .await
    }

    async fn get_pair(&self, name: &str) -> Result<MarketPairHandle> {
        self.exchange_info.get_pair(name)
    }
}

#[async_trait]
impl ExchangeMarketData for Binance {
    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse> {
        self.client
            .get_depth(req.market_pair.as_str(), None)
            .await
            .map(Into::into)
    }

    async fn get_price_ticker(&self, req: &GetPriceTickerRequest) -> Result<Ticker> {
        self.client
            .get_price(&req.market_pair)
            .await
            .map(Into::into)
    }

    async fn get_historic_rates(&self, req: &GetHistoricRatesRequest) -> Result<Vec<Candle>> {
        let params = req.into();

        self.client
            .get_klines(&params)
            .await
            .map(|KlineSummaries::AllKlineSummaries(v)| v.into_iter().map(Into::into).collect())
    }

    async fn get_historic_trades(&self, _req: &GetHistoricTradesRequest) -> Result<Vec<Trade>> {
        unimplemented!("Only implemented for Nash right now");
    }
}

#[async_trait]
impl ExchangeAccount for Binance {
    async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order> {
        let pair = self.exchange_info.get_pair(&req.market_pair)?.read()?;
        self.client
            .limit_buy(
                pair,
                req.size,
                req.price,
                model::TimeInForce::from(req.time_in_force),
                req.post_only,
            )
            .await
            .map(Into::into)
    }
    async fn limit_sell(&self, req: &OpenLimitOrderRequest) -> Result<Order> {
        let pair = self.exchange_info.get_pair(&req.market_pair)?.read()?;
        self.client
            .limit_sell(
                pair,
                req.size,
                req.price,
                model::TimeInForce::from(req.time_in_force),
                req.post_only,
            )
            .await
            .map(Into::into)
    }

    async fn market_buy(&self, req: &OpenMarketOrderRequest) -> Result<Order> {
        let pair = self.exchange_info.get_pair(&req.market_pair)?.read()?;
        self.client.market_buy(pair, req.size).await.map(Into::into)
    }
    async fn market_sell(&self, req: &OpenMarketOrderRequest) -> Result<Order> {
        let pair = self.exchange_info.get_pair(&req.market_pair)?.read()?;
        self.client
            .market_sell(pair, req.size)
            .await
            .map(Into::into)
    }
    async fn cancel_order(&self, req: &CancelOrderRequest) -> Result<OrderCanceled> {
        if let Some(pair) = req.market_pair.as_ref() {
            let u64_id = req
                .id
                .parse::<u64>()
                .expect("binance order id did not parse as u64");
            self.client
                .cancel_order(pair.as_ref(), u64_id)
                .await
                .map(Into::into)
        } else {
            Err(OpenLimitError::MissingParameter(
                "pair parameter is required.".to_string(),
            ))
        }
    }
    async fn cancel_all_orders(&self, req: &CancelAllOrdersRequest) -> Result<Vec<OrderCanceled>> {
        if let Some(pair) = req.market_pair.as_ref() {
            self.client
                .cancel_all_orders(pair)
                .await
                .map(|v| v.into_iter().map(Into::into).collect())
        } else {
            Err(OpenLimitError::MissingParameter(
                "pair parameter is required.".to_string(),
            ))
        }
    }
    async fn get_all_open_orders(&self) -> Result<Vec<Order>> {
        self.client
            .get_all_open_orders()
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_order_history(&self, req: &GetOrderHistoryRequest) -> Result<Vec<Order>> {
        let req = model::AllOrderReq::try_from(req)?;
        self.client
            .get_all_orders(&req)
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_trade_history(&self, req: &TradeHistoryRequest) -> Result<Vec<Trade>> {
        let req = model::TradeHistoryReq::try_from(req)?;
        self.client
            .trade_history(&req)
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_account_balances(&self, _paginator: Option<Paginator>) -> Result<Vec<Balance>> {
        self.client
            .get_account()
            .await
            .map(|v| v.balances.into_iter().map(Into::into).collect())
    }

    async fn get_order(&self, req: &GetOrderRequest) -> Result<Order> {
        let pair = req.market_pair.clone().ok_or_else(|| {
            OpenLimitError::MissingParameter("market_pair parameter is required.".to_string())
        })?;
        let u64_id = req
            .id
            .parse::<u64>()
            .expect("binance order id did not parse as u64");
        self.client.get_order(&pair, u64_id).await.map(Into::into)
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

impl From<model::websocket::Depth> for OrderBookResponse {
    fn from(depth: model::websocket::Depth) -> Self {
        Self {
            last_update_id: Some(depth.final_update_id),
            bids: depth.bids.into_iter().map(Into::into).collect(),
            asks: depth.asks.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<model::websocket::TradeMessage> for Vec<Trade> {
    fn from(trade_message: model::websocket::TradeMessage) -> Self {
        vec![Trade {
            id: trade_message.trade_id.to_string(),
            order_id: trade_message.buyer_order_id.to_string(),
            market_pair: trade_message.symbol,
            price: trade_message.price,
            qty: trade_message.qty,
            fees: None,
            side: match trade_message.is_buyer_maker {
                true => Side::Buy,
                false => Side::Sell,
            },
            liquidity: None,
            created_at: trade_message.event_time,
        }]
    }
}

impl From<TradeMessage> for Trade {
    fn from(trade: TradeMessage) -> Self {
        Self {
            id: trade.trade_id.to_string(),
            order_id: trade.buyer_order_id.to_string(),
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

impl From<model::Order> for Order {
    fn from(order: model::Order) -> Self {
        let order_type = match order.type_name.as_str() {
            ORDER_TYPE_LIMIT => OrderType::Limit,
            ORDER_TYPE_MARKET => OrderType::Market,
            _ => OrderType::Unknown,
        };

        Self {
            id: order.order_id.to_string(),
            market_pair: order.symbol,
            client_order_id: Some(order.client_order_id),
            created_at: order.time,
            order_type,
            side: order.side.into(),
            status: order.status.into(),
            price: Some(order.price),
            size: order.orig_qty,
        }
    }
}

impl From<model::OrderCanceled> for OrderCanceled {
    fn from(order: model::OrderCanceled) -> Self {
        Self {
            id: order.order_id.to_string(),
        }
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

impl From<model::TradeHistory> for Trade {
    fn from(trade_history: model::TradeHistory) -> Self {
        Self {
            id: trade_history.id.to_string(),
            order_id: trade_history.order_id.to_string(),
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

impl TryFrom<&GetOrderHistoryRequest> for model::AllOrderReq {
    type Error = OpenLimitError;
    fn try_from(req: &GetOrderHistoryRequest) -> Result<Self> {
        Ok(Self {
            paginator: req.paginator.clone().map(|p| p.into()),
            symbol: req.market_pair.clone().ok_or_else(|| {
                OpenLimitError::MissingParameter("market_pair parameter is required.".to_string())
            })?,
        })
    }
}

impl TryFrom<&TradeHistoryRequest> for model::TradeHistoryReq {
    type Error = OpenLimitError;
    fn try_from(trade_history: &TradeHistoryRequest) -> Result<Self> {
        Ok(Self {
            paginator: trade_history.paginator.clone().map(|p| p.into()),
            symbol: trade_history.market_pair.clone().ok_or_else(|| {
                OpenLimitError::MissingParameter("market_pair parameter is required.".to_string())
            })?,
        })
    }
}

impl From<&GetHistoricRatesRequest> for model::KlineParams {
    fn from(req: &GetHistoricRatesRequest) -> Self {
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

impl From<TimeInForce> for model::TimeInForce {
    fn from(tif: TimeInForce) -> Self {
        match tif {
            TimeInForce::GoodTillCancelled => model::TimeInForce::GTC,
            TimeInForce::FillOrKill => model::TimeInForce::FOK,
            TimeInForce::ImmediateOrCancelled => model::TimeInForce::IOC,
            _ => panic!("Binance does not support GoodTillTime policy"),
        }
    }
}

impl From<Paginator> for model::Paginator {
    fn from(paginator: Paginator) -> Self {
        Self {
            from_id: paginator
                .after
                .as_ref()
                .map(|s| s.parse().expect("binance page id did not parse as u64")),
            // TODO: what is this, and why do we reuse "after"?
            order_id: paginator
                .after
                .map(|s| s.parse().expect("binance order id did not parse as u64")),
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
