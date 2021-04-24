use crate::{
    errors::{MissingImplementationContent, OpenLimitsError},
    exchange::Exchange,
    exchange::ExchangeAccount,
    exchange::ExchangeMarketData,
    exchange_info::ExchangeInfo,
    exchange_info::MarketPairHandle,
    exchange_info::{ExchangeInfoRetrieval, MarketPair},
    exchange_ws::ExchangeWs,
    exchange_ws::Subscriptions,
    model::websocket::OpenLimitsWebSocketMessage,
    model::{
        websocket::{Subscription, WebSocketResponse},
        AccountFees, AskBid, Balance, CancelAllOrdersRequest, CancelOrderRequest, Candle,
        GetHistoricRatesRequest, GetHistoricTradesRequest, GetOrderHistoryRequest, GetOrderRequest,
        GetPriceTickerRequest, Interval, Liquidity, OpenLimitOrderRequest, OpenMarketOrderRequest,
        Order, OrderBookRequest, OrderBookResponse, OrderCanceled, OrderStatus, OrderType,
        Paginator, Side, Ticker, TimeInForce, Trade, TradeHistoryRequest,
    },
    shared::{timestamp_to_utc_datetime, Result},
};
use async_trait::async_trait;
use chrono::Utc;
pub use nash_native_client::{Client, Environment};
use rust_decimal::prelude::*;
use std::convert::{TryFrom, TryInto};

pub struct Nash {
    transport: Client,
    exchange_info: ExchangeInfo,
}

#[derive(Clone)]
pub struct NashCredentials {
    pub secret: String,
    pub session: String,
}

pub struct NashParameters {
    pub affiliate_code: Option<String>,
    pub credentials: Option<NashCredentials>,
    pub client_id: u64,
    pub environment: Environment,
    pub timeout: Duration,
    pub sign_states_loop_interval: Option<Duration>,
}

impl Clone for NashParameters {
    fn clone(&self) -> Self {
        NashParameters {
            affiliate_code: self.affiliate_code.clone(),
            credentials: self.credentials.clone(),
            client_id: self.client_id,
            environment: match self.environment {
                Environment::Production => Environment::Production,
                Environment::Sandbox => Environment::Sandbox,
                Environment::Dev(s) => Environment::Dev(s),
            },
            timeout: self.timeout,
            sign_states_loop_interval: self.sign_states_loop_interval,
        }
    }
}

async fn client_from_params_failable(params: NashParameters) -> Result<Client> {
    let client = match params.credentials {
        Some(credentials) => {
            Client::from_keys(
                &credentials.secret,
                &credentials.session,
                params.affiliate_code,
                false,
                params.client_id,
                params.environment,
                params.timeout,
            )
            .await?
        }
        None => {
            Client::from_keys_path(
                None,
                None,
                false,
                params.client_id,
                params.environment,
                params.timeout,
            )
            .await?
        }
    };

    if let Some(interval) = params.sign_states_loop_interval {
        client.start_background_sign_states_loop(interval);
    }

    Ok(client)
}

#[async_trait]
impl Exchange for Nash {
    type InitParams = NashParameters;
    type InnerClient = Client;

    async fn new(params: Self::InitParams) -> Result<Self> {
        Ok(Self {
            exchange_info: ExchangeInfo::new(),
            transport: client_from_params_failable(params).await?,
        })
    }

    fn inner_client(&self) -> Option<&Self::InnerClient> {
        Some(&self.transport)
    }
}

#[async_trait]
impl ExchangeMarketData for Nash {
    async fn get_historic_rates(&self, req: &GetHistoricRatesRequest) -> Result<Vec<Candle>> {
        let req: nash_protocol::protocol::list_candles::ListCandlesRequest = req.try_into()?;

        let resp = self.transport.run(req).await;

        let resp: nash_protocol::protocol::list_candles::ListCandlesResponse =
            Nash::unwrap_response::<nash_protocol::protocol::list_candles::ListCandlesResponse>(
                resp,
            )?;

        Ok(resp.candles.into_iter().map(Into::into).collect())
    }

    async fn get_historic_trades(&self, req: &GetHistoricTradesRequest) -> Result<Vec<Trade>> {
        let req: nash_protocol::protocol::list_trades::ListTradesRequest = req.try_into()?;
        let resp = self.transport.run(req).await;

        let resp: nash_protocol::protocol::list_trades::ListTradesResponse = Nash::unwrap_response::<
            nash_protocol::protocol::list_trades::ListTradesResponse,
        >(resp)?;

        Ok(resp.trades.into_iter().map(Into::into).collect())
    }

    async fn get_price_ticker(&self, req: &GetPriceTickerRequest) -> Result<Ticker> {
        let req: nash_protocol::protocol::get_ticker::TickerRequest = req.into();
        let resp = self.transport.run(req).await;
        Ok(
            Nash::unwrap_response::<nash_protocol::protocol::get_ticker::TickerResponse>(resp)?
                .into(),
        )
    }

    async fn order_book(&self, req: &OrderBookRequest) -> Result<OrderBookResponse> {
        let req: nash_protocol::protocol::orderbook::OrderbookRequest = req.into();
        let resp = self.transport.run(req).await;
        Ok(
            Nash::unwrap_response::<nash_protocol::protocol::orderbook::OrderbookResponse>(resp)?
                .into(),
        )
    }
}

#[async_trait]
impl ExchangeAccount for Nash {
    async fn cancel_all_orders(&self, req: &CancelAllOrdersRequest) -> Result<Vec<OrderCanceled>> {
        let req: nash_protocol::protocol::cancel_all_orders::CancelAllOrders = req.into();
        self.transport.run_http(req).await?;
        Ok(vec![])
    }

    async fn cancel_order(&self, req: &CancelOrderRequest) -> Result<OrderCanceled> {
        let req: nash_protocol::protocol::cancel_order::CancelOrderRequest = req.into();
        let resp = self.transport.run_http(req).await;
        Ok(
            Nash::unwrap_response::<nash_protocol::protocol::cancel_order::CancelOrderResponse>(
                resp,
            )?
            .into(),
        )
    }

    async fn get_account_balances(&self, _paginator: Option<Paginator>) -> Result<Vec<Balance>> {
        let req = nash_protocol::protocol::list_account_balances::ListAccountBalancesRequest {
            filter: None,
        };
        let resp = self.transport.run(req).await;

        let resp: nash_protocol::protocol::list_account_balances::ListAccountBalancesResponse =
            Nash::unwrap_response::<
                nash_protocol::protocol::list_account_balances::ListAccountBalancesResponse,
            >(resp)?;

        let mut balances = Vec::new();
        for asset in resp.state_channel.keys() {
            let free = Decimal::from_str(
                &resp
                    .state_channel
                    .get(asset)
                    .expect("Couldn't get asset.")
                    .to_string(),
            )
            .expect("Couldn't parse Decimal from string.");
            let in_orders = Decimal::from_str(
                &resp
                    .in_orders
                    .get(asset)
                    .expect("Couldn't get asset")
                    .to_string(),
            )
            .expect("Couldn't parse Decimal from string.");
            let total = free + in_orders;
            balances.push(Balance {
                asset: asset.name().to_string(),
                total,
                free,
            });
        }

        Ok(balances)
    }

    async fn get_account_fees(&self) -> Result<AccountFees> {
        Err(OpenLimitsError::MissingImplementation(
            MissingImplementationContent {
                message: "'get_account_fees' for nash is not implemented".to_string(),
            },
        ))
    }

    async fn get_all_open_orders(&self) -> Result<Vec<Order>> {
        let req = nash_protocol::protocol::list_account_orders::ListAccountOrdersRequest {
            market: Default::default(),
            before: None,
            buy_or_sell: None,
            limit: Some(100),
            status: Some(vec![nash_protocol::types::OrderStatus::Open]),
            order_type: None,
            range: None,
        };

        let resp = self.transport.run(req).await;

        let resp: nash_protocol::protocol::list_account_orders::ListAccountOrdersResponse =
            Nash::unwrap_response::<
                nash_protocol::protocol::list_account_orders::ListAccountOrdersResponse,
            >(resp)?;

        Ok(resp.orders.into_iter().map(Into::into).collect())
    }

    async fn get_order_history(&self, req: &GetOrderHistoryRequest) -> Result<Vec<Order>> {
        let req: nash_protocol::protocol::list_account_orders::ListAccountOrdersRequest =
            req.try_into()?;

        let resp = self.transport.run(req).await;

        let resp: nash_protocol::protocol::list_account_orders::ListAccountOrdersResponse =
            Nash::unwrap_response::<
                nash_protocol::protocol::list_account_orders::ListAccountOrdersResponse,
            >(resp)?;

        Ok(resp.orders.into_iter().map(Into::into).collect())
    }

    async fn get_trade_history(&self, req: &TradeHistoryRequest) -> Result<Vec<Trade>> {
        let req: nash_protocol::protocol::list_account_trades::ListAccountTradesRequest =
            req.try_into()?;

        let resp = self.transport.run(req).await;

        let resp: nash_protocol::protocol::list_account_trades::ListAccountTradesResponse =
            Nash::unwrap_response::<
                nash_protocol::protocol::list_account_trades::ListAccountTradesResponse,
            >(resp)?;

        Ok(resp.trades.into_iter().map(Into::into).collect())
    }

    async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order> {
        let req: nash_protocol::protocol::place_order::LimitOrderRequest =
            Nash::convert_limit_order(req, nash_protocol::types::BuyOrSell::Buy);

        let resp = self.transport.run_http(req).await;

        Ok(
            Nash::unwrap_response::<nash_protocol::protocol::place_order::PlaceOrderResponse>(
                resp,
            )?
            .into(),
        )
    }

    async fn limit_sell(&self, req: &OpenLimitOrderRequest) -> Result<Order> {
        let req: nash_protocol::protocol::place_order::LimitOrderRequest =
            Nash::convert_limit_order(req, nash_protocol::types::BuyOrSell::Sell);
        let resp = self.transport.run_http(req).await;

        Ok(
            Nash::unwrap_response::<nash_protocol::protocol::place_order::PlaceOrderResponse>(
                resp,
            )?
            .into(),
        )
    }

    async fn market_sell(&self, req: &OpenMarketOrderRequest) -> Result<Order> {
        let req: nash_protocol::protocol::place_order::MarketOrderRequest =
            Nash::convert_market_request(req);

        let resp = self.transport.run_http(req).await;
        Ok(
            Nash::unwrap_response::<nash_protocol::protocol::place_order::PlaceOrderResponse>(
                resp,
            )?
            .into(),
        )
    }

    async fn market_buy(&self, _: &OpenMarketOrderRequest) -> Result<Order> {
        unimplemented!("Market buys are not supported by nash. A market buy can be simulated by placing a market sell in the inverse market. Market buy in btc_usdc should be translated to a market sell in usdc_btc.")
    }

    async fn get_order(&self, req: &GetOrderRequest) -> Result<Order> {
        let req: nash_protocol::protocol::get_account_order::GetAccountOrderRequest = req.into();
        let resp = self.transport.run(req).await;
        let resp = Nash::unwrap_response::<
            nash_protocol::protocol::get_account_order::GetAccountOrderResponse,
        >(resp)?;
        Ok(resp.order.into())
    }
}

impl Nash {
    pub fn unwrap_response<T>(
        resp: std::result::Result<
            nash_protocol::protocol::ResponseOrError<T>,
            nash_protocol::errors::ProtocolError,
        >,
    ) -> Result<T> {
        match resp {
            Ok(resp) => resp
                .response_or_error()
                .map_err(OpenLimitsError::NashProtocolError),
            Err(err) => Err(OpenLimitsError::NashProtocolError(err)),
        }
    }

    pub fn convert_limit_order(
        req: &OpenLimitOrderRequest,
        buy_or_sell: nash_protocol::types::BuyOrSell,
    ) -> nash_protocol::protocol::place_order::LimitOrderRequest {
        nash_protocol::protocol::place_order::LimitOrderRequest {
            client_order_id: None,
            cancellation_policy: nash_protocol::types::OrderCancellationPolicy::from(
                req.time_in_force,
            ),
            allow_taker: !req.post_only,
            market: req.market_pair.clone(),
            buy_or_sell,
            amount: format!("{}", req.size),
            price: format!("{}", req.price),
        }
    }

    pub fn convert_market_request(
        req: &OpenMarketOrderRequest,
    ) -> nash_protocol::protocol::place_order::MarketOrderRequest {
        nash_protocol::protocol::place_order::MarketOrderRequest {
            client_order_id: None,
            market: req.market_pair.clone(),
            amount: format!("{}", req.size),
        }
    }
}

impl From<&OrderBookRequest> for nash_protocol::protocol::orderbook::OrderbookRequest {
    fn from(req: &OrderBookRequest) -> Self {
        let market = req.market_pair.clone();
        Self { market }
    }
}

impl From<nash_protocol::protocol::orderbook::OrderbookResponse> for OrderBookResponse {
    fn from(book: nash_protocol::protocol::orderbook::OrderbookResponse) -> Self {
        Self {
            update_id: Some(book.update_id as u64),
            last_update_id: Some(book.last_update_id as u64),
            bids: book.bids.into_iter().map(Into::into).collect(),
            asks: book.asks.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<nash_protocol::types::OrderbookOrder> for AskBid {
    fn from(resp: nash_protocol::types::OrderbookOrder) -> Self {
        let price = Decimal::from_str(&resp.price).expect("Couldn't parse Decimal from string.");
        let qty = Decimal::from_str(&resp.amount.to_string())
            .expect("Couldn't parse Decimal from string.");
        Self { price, qty }
    }
}

impl From<&CancelOrderRequest> for nash_protocol::protocol::cancel_order::CancelOrderRequest {
    fn from(req: &CancelOrderRequest) -> Self {
        // TODO: why this param?
        let market = req.market_pair.clone().expect("Couldn't get market_pair.");

        Self {
            market,
            order_id: req.id.clone(),
        }
    }
}

impl From<nash_protocol::protocol::cancel_order::CancelOrderResponse> for OrderCanceled {
    fn from(resp: nash_protocol::protocol::cancel_order::CancelOrderResponse) -> Self {
        Self { id: resp.order_id }
    }
}

impl From<&CancelAllOrdersRequest> for nash_protocol::protocol::cancel_all_orders::CancelAllOrders {
    fn from(req: &CancelAllOrdersRequest) -> Self {
        // TODO: why is this required param for Nash?
        let market = req
            .market_pair
            .clone()
            .expect("Market pair is a required param for Nash");
        Self { market }
    }
}

impl From<nash_protocol::types::OrderType> for OrderType {
    fn from(order_type: nash_protocol::types::OrderType) -> Self {
        match order_type {
            nash_protocol::types::OrderType::Limit => OrderType::Limit,
            nash_protocol::types::OrderType::Market => OrderType::Market,
            nash_protocol::types::OrderType::StopLimit => OrderType::StopLimit,
            nash_protocol::types::OrderType::StopMarket => OrderType::StopMarket,
        }
    }
}

impl From<nash_protocol::protocol::place_order::PlaceOrderResponse> for Order {
    fn from(resp: nash_protocol::protocol::place_order::PlaceOrderResponse) -> Self {
        Self {
            id: resp.order_id,
            market_pair: resp.market_name,
            client_order_id: None,
            created_at: Some(resp.placed_at.timestamp_millis() as u64),
            order_type: resp.order_type.into(),
            side: resp.buy_or_sell.into(),
            status: resp.status.into(),
            size: Decimal::from(0),
            price: None,
            remaining: None,
            trades: Vec::new(),
        }
    }
}

impl TryFrom<&TradeHistoryRequest>
    for nash_protocol::protocol::list_account_trades::ListAccountTradesRequest
{
    type Error = OpenLimitsError;
    fn try_from(req: &TradeHistoryRequest) -> crate::shared::Result<Self> {
        let (before, limit, range) = try_split_paginator(req.paginator.clone())?;

        Ok(Self {
            market: req.market_pair.clone(),
            before,
            limit,
            range,
        })
    }
}

impl From<nash_protocol::types::Trade> for Trade {
    fn from(resp: nash_protocol::types::Trade) -> Self {
        let qty = Decimal::from_str(&resp.amount.to_string())
            .expect("Couldn't parse Decimal from string.");
        let price = Decimal::from_str(&resp.limit_price.to_string())
            .expect("Couldn't parse Decimal from string.");

        let fees = match resp.account_side {
            nash_protocol::types::AccountTradeSide::Taker => {
                Decimal::from_str(&resp.taker_fee.to_string())
                    .expect("Couldn't parse Decimal from string.")
            }
            _ => Decimal::from(0),
        };

        let (buyer_order_id, seller_order_id) = match resp.direction {
            nash_protocol::types::BuyOrSell::Buy => (resp.taker_order_id, resp.maker_order_id),
            nash_protocol::types::BuyOrSell::Sell => (resp.maker_order_id, resp.taker_order_id),
        };

        Self {
            id: resp.id,
            created_at: resp.executed_at.timestamp_millis() as u64,
            fees: Some(fees),
            liquidity: Some(resp.account_side.into()),
            market_pair: resp.market.clone(),
            buyer_order_id: Some(buyer_order_id),
            seller_order_id: Some(seller_order_id),
            price,
            qty,
            side: resp.direction.into(),
        }
    }
}

impl From<nash_protocol::types::BuyOrSell> for Side {
    fn from(side: nash_protocol::types::BuyOrSell) -> Self {
        match side {
            nash_protocol::types::BuyOrSell::Buy => Side::Buy,
            nash_protocol::types::BuyOrSell::Sell => Side::Sell,
        }
    }
}

impl From<nash_protocol::types::AccountTradeSide> for Liquidity {
    fn from(side: nash_protocol::types::AccountTradeSide) -> Self {
        match side {
            nash_protocol::types::AccountTradeSide::Taker => Liquidity::Taker,
            _ => Liquidity::Maker,
        }
    }
}

impl TryFrom<&GetHistoricRatesRequest>
    for nash_protocol::protocol::list_candles::ListCandlesRequest
{
    type Error = OpenLimitsError;
    fn try_from(req: &GetHistoricRatesRequest) -> crate::shared::Result<Self> {
        let (before, limit, range) = try_split_paginator(req.paginator.clone())?;

        Ok(Self {
            market: req.market_pair.clone(),
            chronological: None,
            before,
            interval: Some(
                req.interval
                    .try_into()
                    .expect("Couldn't convert Interval to CandleInterval."),
            ),
            limit,
            range,
        })
    }
}

fn try_split_paginator(
    paginator: Option<Paginator>,
) -> crate::shared::Result<(
    Option<String>,
    Option<i64>,
    Option<nash_protocol::types::DateTimeRange>,
)> {
    Ok(match paginator {
        Some(paginator) => (
            paginator.before,
            match paginator.limit {
                Some(v) => Some(i64::try_from(v).map_err(|_| {
                    OpenLimitsError::InvalidParameter(
                        "Couldn't convert paginator limit to i64".to_string(),
                    )
                })?),
                None => None,
            },
            if paginator.start_time.is_some() && paginator.end_time.is_some() {
                Some(nash_protocol::types::DateTimeRange {
                    start: paginator.start_time.map(timestamp_to_utc_datetime).unwrap(),
                    stop: paginator.end_time.map(timestamp_to_utc_datetime).unwrap(),
                })
            } else {
                None
            },
        ),
        None => (None, None, None),
    })
}

impl TryFrom<&GetHistoricTradesRequest>
    for nash_protocol::protocol::list_trades::ListTradesRequest
{
    type Error = OpenLimitsError;
    fn try_from(req: &GetHistoricTradesRequest) -> crate::shared::Result<Self> {
        let market = req.market_pair.clone();
        let (before, limit, _) = try_split_paginator(req.paginator.clone())?;
        //FIXME: Some issues with the graphql protocol for the market to be non nil
        Ok(Self {
            market,
            before,
            limit,
        })
    }
}

impl TryFrom<Interval> for nash_protocol::types::CandleInterval {
    type Error = OpenLimitsError;
    fn try_from(interval: Interval) -> crate::shared::Result<Self> {
        match interval {
            Interval::OneMinute => Ok(nash_protocol::types::CandleInterval::OneMinute),
            Interval::FiveMinutes => Ok(nash_protocol::types::CandleInterval::FiveMinute),
            Interval::FifteenMinutes => Ok(nash_protocol::types::CandleInterval::FifteenMinute),
            Interval::ThirtyMinutes => Ok(nash_protocol::types::CandleInterval::ThirtyMinute),
            Interval::OneHour => Ok(nash_protocol::types::CandleInterval::OneHour),
            Interval::SixHours => Ok(nash_protocol::types::CandleInterval::SixHour),
            Interval::TwelveHours => Ok(nash_protocol::types::CandleInterval::TwelveHour),
            Interval::OneDay => Ok(nash_protocol::types::CandleInterval::OneDay),
            _ => {
                let err = MissingImplementationContent {
                    message: String::from("Not supported interval"),
                };
                Err(OpenLimitsError::MissingImplementation(err))
            }
        }
    }
}

impl From<nash_protocol::types::Candle> for Candle {
    fn from(candle: nash_protocol::types::Candle) -> Self {
        let close = Decimal::from_str(&candle.close_price.to_string())
            .expect("Couldn't parse Decimal from string.");
        let high = Decimal::from_str(&candle.high_price.to_string())
            .expect("Couldn't parse Decimal from string.");
        let low = Decimal::from_str(&candle.low_price.to_string())
            .expect("Couldn't parse Decimal from string.");
        let open = Decimal::from_str(&candle.open_price.to_string())
            .expect("Couldn't parse Decimal from string.");
        let volume = Decimal::from_str(&candle.a_volume.to_string())
            .expect("Couldn't parse Decimal from string.");

        Self {
            close,
            high,
            low,
            open,
            time: candle.interval_start.timestamp_millis() as u64,
            volume,
        }
    }
}

impl TryFrom<&GetOrderHistoryRequest>
    for nash_protocol::protocol::list_account_orders::ListAccountOrdersRequest
{
    type Error = OpenLimitsError;
    fn try_from(req: &GetOrderHistoryRequest) -> crate::shared::Result<Self> {
        let (before, limit, range) = try_split_paginator(req.paginator.clone())?;

        Ok(Self {
            market: req.market_pair.clone(),
            before,
            limit,
            range,
            buy_or_sell: None,
            order_type: None,
            status: match req.order_status.clone() {
                Some(v) => Some(
                    v.into_iter()
                        .map(TryInto::try_into)
                        .collect::<Result<Vec<nash_protocol::types::OrderStatus>>>()?,
                ),
                None => None,
            },
        })
    }
}

impl From<nash_protocol::types::Order> for Order {
    fn from(order: nash_protocol::types::Order) -> Self {
        let size = Decimal::from_str(&order.amount_placed.to_string())
            .expect("Couldn't parse Decimal from string.");
        let price = order
            .limit_price
            .map(|p| Decimal::from_str(&p.to_string()).unwrap());
        let remaining = Some(
            Decimal::from_str(&order.amount_remaining.to_string())
                .expect("Couldn't parse Decimal from string."),
        );

        Self {
            id: order.id,
            market_pair: order.market.clone(),
            client_order_id: None,
            created_at: Some(order.placed_at.timestamp_millis() as u64),
            order_type: order.order_type.into(),
            side: order.buy_or_sell.into(),
            status: order.status.into(),
            size,
            price,
            remaining,
            trades: order.trades.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<nash_protocol::types::OrderStatus> for OrderStatus {
    fn from(status: nash_protocol::types::OrderStatus) -> Self {
        match status {
            nash_protocol::types::OrderStatus::Filled => OrderStatus::Filled,
            nash_protocol::types::OrderStatus::Open => OrderStatus::Open,
            nash_protocol::types::OrderStatus::Canceled => OrderStatus::Canceled,
            nash_protocol::types::OrderStatus::Pending => OrderStatus::Pending,
        }
    }
}

impl TryFrom<OrderStatus> for nash_protocol::types::OrderStatus {
    type Error = OpenLimitsError;
    fn try_from(status: OrderStatus) -> crate::shared::Result<Self> {
        Ok(match status {
            OrderStatus::Filled => nash_protocol::types::OrderStatus::Filled,
            OrderStatus::Open => nash_protocol::types::OrderStatus::Open,
            OrderStatus::Canceled => nash_protocol::types::OrderStatus::Canceled,
            OrderStatus::Pending => nash_protocol::types::OrderStatus::Pending,
            _ => {
                return Err(OpenLimitsError::InvalidParameter(
                    "Had invalid order status for Nash".to_string(),
                ))
            }
        })
    }
}

impl From<&GetPriceTickerRequest> for nash_protocol::protocol::get_ticker::TickerRequest {
    fn from(req: &GetPriceTickerRequest) -> Self {
        let market = req.market_pair.clone();

        Self { market }
    }
}

impl From<nash_protocol::protocol::get_ticker::TickerResponse> for Ticker {
    fn from(resp: nash_protocol::protocol::get_ticker::TickerResponse) -> Self {
        let mut price = None;
        if resp.best_ask_price.is_some() && resp.best_bid_price.is_some() {
            let ask = Decimal::from_str(&resp.best_ask_price.unwrap().to_string())
                .expect("Couldn't parse Decimal from string.");
            let bid = Decimal::from_str(&resp.best_bid_price.unwrap().to_string())
                .expect("Couldn't parse Decimal from string.");
            price = Some((ask + bid) / Decimal::from(2));
        }
        let mut price_24h = None;
        if resp.high_price_24h.is_some() && resp.low_price_24h.is_some() {
            let day_high = Decimal::from_str(
                &resp
                    .high_price_24h
                    .expect("Couldn't get high price 24h.")
                    .to_string(),
            )
            .expect("Couldn't parse Decimal from string.");
            let day_low = Decimal::from_str(
                &resp
                    .low_price_24h
                    .expect("Couldn't get low price 24h.")
                    .to_string(),
            )
            .expect("Couldn't parse Decimal from string.");
            price_24h = Some((day_high + day_low) / Decimal::from(2));
        }
        Self { price, price_24h }
    }
}

impl From<&GetOrderRequest> for nash_protocol::protocol::get_account_order::GetAccountOrderRequest {
    fn from(req: &GetOrderRequest) -> Self {
        Self {
            order_id: req.id.clone(),
        }
    }
}

use crate::model::websocket::AccountOrders;
use futures::stream::{BoxStream, SelectAll, Stream, StreamExt};
use nash_protocol::protocol::subscriptions::updated_account_orders::SubscribeAccountOrders;
use nash_protocol::protocol::{
    subscriptions::{SubscriptionRequest, SubscriptionResponse},
    ResponseOrError,
};
use nash_protocol::types::{BuyOrSell, DateTimeRange};
use std::{pin::Pin, task::Context, task::Poll};
use tokio::time::Duration;

pub struct NashWebsocket {
    pub client: Client,
}

impl Stream for NashWebsocket {
    type Item = std::result::Result<
        ResponseOrError<nash_protocol::protocol::subscriptions::SubscriptionResponse>,
        nash_protocol::errors::ProtocolError,
    >;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.client.poll_next_unpin(cx)
    }
}

#[async_trait]
impl ExchangeWs for NashWebsocket {
    type InitParams = NashParameters;

    type Subscription = SubscriptionRequest;
    type Response = SubscriptionResponseWrapper;

    async fn new(params: Self::InitParams) -> Result<Self> {
        Ok(Self {
            client: client_from_params_failable(params).await?,
        })
    }

    async fn disconnect(&self) {
        self.client.disconnect().await;
    }

    async fn create_stream_specific(
        &self,
        subscriptions: Subscriptions<Self::Subscription>,
    ) -> Result<BoxStream<'static, Result<Self::Response>>> {
        let mut streams = SelectAll::new();

        for subscription in subscriptions.into_iter() {
            let stream = self.client.subscribe_protocol(subscription).await?;
            streams.push(tokio_stream::wrappers::UnboundedReceiverStream::new(stream));
        }

        let s = streams.map(|message| match message {
            Ok(msg) => match msg {
                ResponseOrError::Response(resp) => Ok(SubscriptionResponseWrapper(resp.data)),
                ResponseOrError::Error(resp) => {
                    let f = resp
                        .errors
                        .iter()
                        .map(|f| f.message.clone())
                        .collect::<Vec<String>>()
                        .join("\n");
                    Err(OpenLimitsError::NotParsableResponse(f))
                }
            },
            Err(_) => Err(OpenLimitsError::SocketError()),
        });

        Ok(s.boxed())
    }
}

impl From<Side> for BuyOrSell {
    fn from(side: Side) -> Self {
        match side {
            Side::Buy => BuyOrSell::Buy,
            Side::Sell => BuyOrSell::Sell,
        }
    }
}

impl TryFrom<OrderType> for nash_protocol::types::OrderType {
    type Error = OpenLimitsError;
    fn try_from(order_type: OrderType) -> Result<Self> {
        match order_type {
            OrderType::Limit => Ok(Self::Limit),
            OrderType::Market => Ok(Self::Market),
            OrderType::StopLimit => Ok(Self::StopLimit),
            OrderType::StopMarket => Ok(Self::StopMarket),
            OrderType::Unknown => Err(OpenLimitsError::InvalidParameter(
                "Had invalid order type for Nash".to_string(),
            )),
        }
    }
}

impl From<AccountOrders> for SubscribeAccountOrders {
    fn from(account_orders: AccountOrders) -> Self {
        Self {
            market: account_orders.market.clone(),
            order_type: account_orders.order_type.map(|x| {
                x.iter()
                    .cloned()
                    .map(|x| x.try_into().ok())
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap())
                    .collect()
            }),
            range: account_orders.range.map(|range| DateTimeRange {
                start: timestamp_to_utc_datetime(range.start),
                stop: timestamp_to_utc_datetime(range.end),
            }),
            buy_or_sell: account_orders.buy_or_sell.map(|x| x.into()),
            status: account_orders.status.map(|x| {
                x.iter()
                    .cloned()
                    .map(|x| x.try_into().ok())
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap())
                    .collect()
            }),
        }
    }
}

impl From<Subscription> for nash_protocol::protocol::subscriptions::SubscriptionRequest {
    fn from(sub: Subscription) -> Self {
        match sub {
            Subscription::OrderBookUpdates(market) => Self::Orderbook(
                nash_protocol::protocol::subscriptions::updated_orderbook::SubscribeOrderbook {
                    market,
                },
            ),
            Subscription::Trades(market) => Self::Trades(
                nash_protocol::protocol::subscriptions::trades::SubscribeTrades { market },
            ),
            Subscription::AccountOrders(account_orders) => Self::AccountOrders(
                account_orders.into()
            ),
            Subscription::AccountTrades(market_name) => Self::AccountTrades(
                nash_protocol::protocol::subscriptions::new_account_trades::SubscribeAccountTrades {
                    market_name
                }
            ),
            Subscription::AccountBalance(symbol) => Self::AccountBalances(
                nash_protocol::protocol::subscriptions::updated_account_balances::SubscribeAccountBalances {
                    symbol
                }
            ),
            _ => panic!("Not supported Subscription"),
        }
    }
}

#[derive(Debug)]
pub struct SubscriptionResponseWrapper(SubscriptionResponse);

impl Clone for SubscriptionResponseWrapper {
    fn clone(&self) -> Self {
        match &self.0 {
            SubscriptionResponse::Orderbook(o) => {
                SubscriptionResponseWrapper(SubscriptionResponse::Orderbook(o.clone()))
            }
            SubscriptionResponse::Trades(t) => {
                SubscriptionResponseWrapper(SubscriptionResponse::Trades(t.clone()))
            }
            SubscriptionResponse::Ticker(ticker) => {
                SubscriptionResponseWrapper(SubscriptionResponse::Ticker(ticker.clone()))
            }
            SubscriptionResponse::AccountBalances(balances) => {
                SubscriptionResponseWrapper(SubscriptionResponse::AccountBalances(balances.clone()))
            }
            SubscriptionResponse::AccountOrders(orders) => {
                SubscriptionResponseWrapper(SubscriptionResponse::AccountOrders(orders.clone()))
            }
            SubscriptionResponse::AccountTrades(trades) => {
                SubscriptionResponseWrapper(SubscriptionResponse::AccountTrades(trades.clone()))
            }
        }
    }
}

impl TryFrom<SubscriptionResponseWrapper> for WebSocketResponse<SubscriptionResponseWrapper> {
    type Error = OpenLimitsError;

    fn try_from(value: SubscriptionResponseWrapper) -> Result<Self> {
        match value.0 {
            SubscriptionResponse::Orderbook(resp) => Ok(WebSocketResponse::Generic(
                OpenLimitsWebSocketMessage::OrderBook(OrderBookResponse {
                    update_id: Some(resp.update_id as u64),
                    last_update_id: Some(resp.last_update_id as u64),
                    asks: resp.asks.into_iter().map(Into::into).collect(),
                    bids: resp.bids.into_iter().map(Into::into).collect(),
                }),
            )),
            SubscriptionResponse::Trades(resp) => {
                let trades = resp.trades.into_iter().map(|x| x.into()).collect();
                Ok(WebSocketResponse::Generic(
                    OpenLimitsWebSocketMessage::Trades(trades),
                ))
            }
            SubscriptionResponse::Ticker(resp) => Ok(WebSocketResponse::Raw(
                SubscriptionResponseWrapper(SubscriptionResponse::Ticker(resp)),
            )),
            SubscriptionResponse::AccountTrades(resp) => Ok(WebSocketResponse::Raw(
                SubscriptionResponseWrapper(SubscriptionResponse::AccountTrades(resp)),
            )),
            SubscriptionResponse::AccountOrders(resp) => Ok(WebSocketResponse::Raw(
                SubscriptionResponseWrapper(SubscriptionResponse::AccountOrders(resp)),
            )),
            SubscriptionResponse::AccountBalances(resp) => Ok(WebSocketResponse::Raw(
                SubscriptionResponseWrapper(SubscriptionResponse::AccountBalances(resp)),
            )),
        }
    }
}

impl From<TimeInForce> for nash_protocol::types::OrderCancellationPolicy {
    fn from(tif: TimeInForce) -> Self {
        match tif {
            TimeInForce::GoodTillCancelled => {
                nash_protocol::types::OrderCancellationPolicy::GoodTilCancelled
            }
            TimeInForce::FillOrKill => nash_protocol::types::OrderCancellationPolicy::FillOrKill,
            TimeInForce::ImmediateOrCancelled => {
                nash_protocol::types::OrderCancellationPolicy::ImmediateOrCancel
            }
            TimeInForce::GoodTillTime(duration) => {
                let expire_time = Utc::now() + duration;
                nash_protocol::types::OrderCancellationPolicy::GoodTilTime(expire_time)
            }
        }
    }
}

impl Nash {
    async fn list_markets(
        &self,
    ) -> Result<nash_protocol::protocol::list_markets::ListMarketsResponse> {
        let response = self
            .transport
            .run(nash_protocol::protocol::list_markets::ListMarketsRequest)
            .await?;
        if let Some(err) = response.error() {
            Err(OpenLimitsError::NashProtocolError(
                // FIXME: handle this better in both nash protocol and openlimits
                nash_protocol::errors::ProtocolError::coerce_static_from_str(&format!(
                    "{:#?}",
                    err
                )),
            ))
        } else {
            Ok(response
                .consume_response()
                .expect("Couldn't consume response.")) // safe unwrap
        }
    }
}

#[async_trait]
impl ExchangeInfoRetrieval for Nash {
    async fn retrieve_pairs(&self) -> Result<Vec<MarketPair>> {
        Ok(self
            .list_markets()
            .await?
            .markets
            .iter()
            .map(|(symbol, v)| MarketPair {
                symbol: symbol.to_string(),
                base: v.asset_a.asset.name().to_string(),
                quote: v.asset_b.asset.name().to_string(),
                base_increment: Decimal::new(1, v.asset_a.precision),
                quote_increment: Decimal::new(1, v.asset_b.precision),
                min_base_trade_size: Some(
                    Decimal::from_str(&format!("{}", &v.min_trade_size_a.amount.value))
                        .expect("Couldn't create Decimal from string."),
                ),
                min_quote_trade_size: Some(
                    Decimal::from_str(&format!("{}", &v.min_trade_size_b.amount.value))
                        .expect("Couldn't create Decimal from string."),
                ),
            })
            .collect())
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
