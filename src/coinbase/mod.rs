pub mod client;
pub mod model;
mod transport;

use crate::{
    errors::OpenLimitError,
    exchange::Exchange,
    exchange::ExchangeAccount,
    exchange::ExchangeMarketData,
    exchange_info::ExchangeInfo,
    exchange_info::{ExchangeInfoRetrieval, MarketPair, MarketPairHandle},
    model::{
        AskBid, Balance, CancelAllOrdersRequest, CancelOrderRequest, Candle,
        GetHistoricRatesRequest, GetHistoricTradesRequest, GetOrderHistoryRequest, GetOrderRequest,
        GetPriceTickerRequest, Interval, Liquidity, OpenLimitOrderRequest, OpenMarketOrderRequest,
        Order, OrderBookRequest, OrderBookResponse, OrderCanceled, OrderStatus, OrderType,
        Paginator, Side, Ticker, TimeInForce, Trade, TradeHistoryRequest,
    },
    shared::{timestamp_to_naive_datetime, Result},
};
use async_trait::async_trait;
use chrono::Duration;
use client::BaseClient;
use std::convert::TryFrom;
use transport::Transport;

#[derive(Clone)]
pub struct Coinbase {
    exchange_info: ExchangeInfo,
    client: BaseClient,
}

#[derive(Clone)]
pub struct CoinbaseCredentials {
    pub api_key: String,
    pub api_secret: String,
    pub passphrase: String,
}

#[derive(Default, Clone)]
pub struct CoinbaseParameters {
    pub sandbox: bool,
    pub credentials: Option<CoinbaseCredentials>,
}

impl CoinbaseParameters {
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
impl Exchange for Coinbase {
    type InitParams = CoinbaseParameters;
    type InnerClient = BaseClient;

    async fn new(parameters: Self::InitParams) -> Self {
        let coinbase = match parameters.credentials {
            Some(credentials) => Coinbase {
                exchange_info: ExchangeInfo::new(),
                client: BaseClient {
                    transport: Transport::with_credential(
                        &credentials.api_key,
                        &credentials.api_secret,
                        &credentials.passphrase,
                        parameters.sandbox,
                    )
                    .expect("Couldn't construct transport."),
                },
            },
            None => Coinbase {
                exchange_info: ExchangeInfo::new(),
                client: BaseClient {
                    transport: Transport::new(parameters.sandbox).expect("Couldn't construct transport."),
                },
            },
        };

        coinbase.refresh_market_info().await.expect("Couldn't refresh market info.");
        coinbase
    }

    fn inner_client(&self) -> Option<&Self::InnerClient> {
        Some(&self.client)
    }
}

#[async_trait]
impl ExchangeInfoRetrieval for Coinbase {
    async fn retrieve_pairs(&self) -> Result<Vec<MarketPair>> {
        self.client.products().await.map(|v| {
            v.into_iter()
                .map(|product| MarketPair {
                    symbol: product.id,
                    base: product.base_currency,
                    quote: product.quote_currency,
                    base_increment: product.base_increment,
                    quote_increment: product.quote_increment,
                    min_base_trade_size: None,
                    min_quote_trade_size: None,
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
impl ExchangeMarketData for Coinbase {
    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse> {
        self.client
            .book::<model::BookRecordL2>(&req.market_pair)
            .await
            .map(Into::into)
    }

    async fn get_price_ticker(&self, req: &GetPriceTickerRequest) -> Result<Ticker> {
        self.client.ticker(&req.market_pair).await.map(Into::into)
    }

    async fn get_historic_rates(&self, req: &GetHistoricRatesRequest) -> Result<Vec<Candle>> {
        let params = model::CandleRequestParams::try_from(req)?;
        self.client
            .candles(&req.market_pair, Some(&params))
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_historic_trades(&self, _req: &GetHistoricTradesRequest) -> Result<Vec<Trade>> {
        unimplemented!("Only implemented for Nash right now");
    }
}

impl From<model::Book<model::BookRecordL2>> for OrderBookResponse {
    fn from(book: model::Book<model::BookRecordL2>) -> Self {
        Self {
            update_id: Some(book.sequence as u64),
            last_update_id: None,
            bids: book.bids.into_iter().map(Into::into).collect(),
            asks: book.asks.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<model::BookRecordL2> for AskBid {
    fn from(bids: model::BookRecordL2) -> Self {
        Self {
            price: bids.price,
            qty: bids.size,
        }
    }
}

impl From<model::Order> for Order {
    fn from(order: model::Order) -> Self {
        let (price, size, order_type) = match order._type {
            model::OrderType::Limit {
                price,
                size,
                time_in_force: _,
            } => (Some(price), size, OrderType::Limit),
            model::OrderType::Market { size, funds: _ } => (None, size, OrderType::Market),
        };

        Self {
            id: order.id,
            market_pair: order.product_id,
            client_order_id: None,
            created_at: Some((order.created_at.timestamp_millis()) as u64),
            order_type,
            side: order.side.into(),
            status: order.status.into(),
            size,
            price,
            remaining: Some(size - order.filled_size),
            trades: Vec::new(),
        }
    }
}

#[async_trait]
impl ExchangeAccount for Coinbase {
    async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order> {
        let pair = self.exchange_info.get_pair(&req.market_pair)?.read()?;
        self.client
            .limit_buy(
                pair,
                req.size,
                req.price,
                model::OrderTimeInForce::from(req.time_in_force.clone()),
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
                model::OrderTimeInForce::from(req.time_in_force.clone()),
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
        self.client
            .cancel_order(req.id.clone(), req.market_pair.as_deref())
            .await
            .map(Into::into)
    }

    async fn cancel_all_orders(&self, req: &CancelAllOrdersRequest) -> Result<Vec<OrderCanceled>> {
        self.client
            .cancel_all_orders(req.market_pair.as_deref())
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_all_open_orders(&self) -> Result<Vec<Order>> {
        let params = model::GetOrderRequest {
            status: Some(String::from("open")),
            paginator: None,
            product_id: None,
        };

        self.client
            .get_orders(Some(&params))
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_order_history(&self, req: &GetOrderHistoryRequest) -> Result<Vec<Order>> {
        let req: model::GetOrderRequest = req.into();

        self.client
            .get_orders(Some(&req))
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_trade_history(&self, req: &TradeHistoryRequest) -> Result<Vec<Trade>> {
        let req = req.into();

        self.client
            .get_fills(Some(&req))
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_account_balances(&self, paginator: Option<Paginator>) -> Result<Vec<Balance>> {
        let paginator: Option<model::Paginator> = paginator.map(|p| p.into());

        self.client
            .get_account(paginator.as_ref())
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
    }

    async fn get_order(&self, req: &GetOrderRequest) -> Result<Order> {
        let id = req.id.clone();

        self.client.get_order(id).await.map(Into::into)
    }
}

impl From<String> for OrderCanceled {
    fn from(id: String) -> Self {
        Self { id }
    }
}

impl From<model::Account> for Balance {
    fn from(account: model::Account) -> Self {
        Self {
            asset: account.currency,
            free: account.available,
            total: account.balance,
        }
    }
}

impl From<model::Fill> for Trade {
    fn from(fill: model::Fill) -> Self {
        Self {
            id: fill.trade_id.to_string(),
            order_id: fill.order_id,
            market_pair: fill.product_id,
            price: fill.price,
            qty: fill.size,
            fees: Some(fill.fee),
            side: match fill.side.as_str() {
                "buy" => Side::Buy,
                _ => Side::Sell,
            },
            liquidity: match fill.liquidity.as_str() {
                "M" => Some(Liquidity::Maker),
                "T" => Some(Liquidity::Taker),
                _ => None,
            },
            created_at: (fill.created_at.timestamp_millis()) as u64,
        }
    }
}

impl From<model::Ticker> for Ticker {
    fn from(ticker: model::Ticker) -> Self {
        Self {
            price: Some(ticker.price),
            price_24h: None,
        }
    }
}

impl From<model::Candle> for Candle {
    fn from(candle: model::Candle) -> Self {
        Self {
            time: candle.time * 1000,
            low: candle.low,
            high: candle.high,
            open: candle.open,
            close: candle.close,
            volume: candle.volume,
        }
    }
}

impl TryFrom<Interval> for u32 {
    type Error = OpenLimitError;
    fn try_from(value: Interval) -> Result<Self> {
        match value {
            Interval::OneMinute => Ok(60),
            Interval::FiveMinutes => Ok(300),
            Interval::FifteenMinutes => Ok(900),
            Interval::OneHour => Ok(3600),
            Interval::SixHours => Ok(21600),
            Interval::OneDay => Ok(86400),
            _ => Err(OpenLimitError::MissingParameter(format!(
                "{:?} is not supported in Coinbase",
                value,
            ))),
        }
    }
}

impl TryFrom<&GetHistoricRatesRequest> for model::CandleRequestParams {
    type Error = OpenLimitError;
    fn try_from(params: &GetHistoricRatesRequest) -> Result<Self> {
        let granularity = u32::try_from(params.interval)?;
        Ok(Self {
            daterange: params.paginator.clone().map(|p| p.into()),
            granularity: Some(granularity),
        })
    }
}

impl From<&GetOrderHistoryRequest> for model::GetOrderRequest {
    fn from(req: &GetOrderHistoryRequest) -> Self {
        Self {
            product_id: req.market_pair.clone(),
            paginator: req.paginator.clone().map(|p| p.into()),
            status: None,
        }
    }
}

impl From<Paginator> for model::Paginator {
    fn from(paginator: Paginator) -> Self {
        Self {
            after: paginator.after.map(|s| s.parse::<u64>().expect("Couldn't parse paginator.")),
            before: paginator.before.map(|s| s.parse::<u64>().expect("Couldn't parse paginator.")),
            limit: paginator.limit,
        }
    }
}

impl From<&Paginator> for model::Paginator {
    fn from(paginator: &Paginator) -> Self {
        Self {
            after: paginator
                .after
                .as_ref()
                .map(|s| s.parse().expect("coinbase page id did not parse as u64")),
            before: paginator
                .before
                .as_ref()
                .map(|s| s.parse().expect("coinbase page id did not parse as u64")),
            limit: paginator.limit,
        }
    }
}

impl From<Paginator> for model::DateRange {
    fn from(paginator: Paginator) -> Self {
        Self {
            start: paginator.start_time.map(timestamp_to_naive_datetime),
            end: paginator.end_time.map(timestamp_to_naive_datetime),
        }
    }
}

impl From<&Paginator> for model::DateRange {
    fn from(paginator: &Paginator) -> Self {
        Self {
            start: paginator.start_time.map(timestamp_to_naive_datetime),
            end: paginator.end_time.map(timestamp_to_naive_datetime),
        }
    }
}

impl From<TimeInForce> for model::OrderTimeInForce {
    fn from(tif: TimeInForce) -> Self {
        match tif {
            TimeInForce::GoodTillCancelled => model::OrderTimeInForce::GTC,
            TimeInForce::FillOrKill => model::OrderTimeInForce::FOK,
            TimeInForce::ImmediateOrCancelled => model::OrderTimeInForce::IOC,
            TimeInForce::GoodTillTime(duration) => {
                let day: Duration = Duration::days(1);
                let hour: Duration = Duration::hours(1);
                let minute: Duration = Duration::minutes(1);

                if duration == day {
                    model::OrderTimeInForce::GTT {
                        cancel_after: model::CancelAfter::Day,
                    }
                } else if duration == hour {
                    model::OrderTimeInForce::GTT {
                        cancel_after: model::CancelAfter::Hour,
                    }
                } else if duration == minute {
                    model::OrderTimeInForce::GTT {
                        cancel_after: model::CancelAfter::Min,
                    }
                } else {
                    panic!("Coinbase only supports durations of 1 day, 1 hour or 1 minute")
                }
            }
        }
    }
}

impl From<&TradeHistoryRequest> for model::GetFillsReq {
    fn from(req: &TradeHistoryRequest) -> Self {
        Self {
            order_id: req.order_id.clone(),
            paginator: req.paginator.clone().map(|p| p.into()),
            product_id: req.market_pair.clone(),
        }
    }
}

impl From<model::OrderSide> for Side {
    fn from(req: model::OrderSide) -> Self {
        match req {
            model::OrderSide::Buy => Side::Buy,
            model::OrderSide::Sell => Side::Sell,
        }
    }
}

impl From<model::OrderStatus> for OrderStatus {
    fn from(req: model::OrderStatus) -> OrderStatus {
        match req {
            model::OrderStatus::Active => OrderStatus::Active,
            model::OrderStatus::Done => OrderStatus::Filled,
            model::OrderStatus::Open => OrderStatus::Open,
            model::OrderStatus::Pending => OrderStatus::Pending,
            model::OrderStatus::Rejected => OrderStatus::Rejected,
        }
    }
}
