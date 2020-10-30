use async_trait::async_trait;
pub use nash_native_client::ws_client::client::Client;
use std::convert::{TryFrom, TryInto};

use crate::{
    errors::{MissingImplementationContent, OpenLimitError},
    exchange::Exchange,
    exchange::ExchangeAccount,
    exchange::ExchangeMarketData,
    exchange_info::ExchangeInfo,
    exchange_info::MarketPairHandle,
    exchange_info::{ExchangeInfoRetrieval, MarketPair},
    exchange_ws::ExchangeWs,
    model::{
        websocket::{OpenLimitsWebsocketMessage, Subscription},
        AskBid, Balance, CancelAllOrdersRequest, CancelOrderRequest, Candle,
        GetHistoricRatesRequest, GetHistoricTradesRequest, GetOrderHistoryRequest, GetOrderRequest,
        GetPriceTickerRequest, Interval, Liquidity, OpenLimitOrderRequest, OpenMarketOrderRequest,
        Order, OrderBookRequest, OrderBookResponse, OrderCanceled, OrderStatus, OrderType,
        Paginator, Side, Ticker, TimeInForce, Trade, TradeHistoryRequest,
    },
    shared::{timestamp_to_utc_datetime, Result},
};
use chrono::Utc;
pub use nash_native_client::ws_client::client::Environment;
use rust_decimal::prelude::*;

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
    pub credentials: Option<NashCredentials>,
    pub client_id: u64,
    pub environment: Environment,
    pub timeout: u64,
}

impl Clone for NashParameters {
    fn clone(&self) -> Self {
        NashParameters {
            credentials: self.credentials.clone(),
            client_id: self.client_id,
            environment: match self.environment {
                Environment::Production => Environment::Production,
                Environment::Sandbox => Environment::Sandbox,
                Environment::Dev(s) => Environment::Dev(s),
            },
            timeout: self.timeout,
        }
    }
}

#[async_trait]
impl Exchange for Nash {
    type InitParams = NashParameters;
    type InnerClient = Client;

    async fn new(parameters: Self::InitParams) -> Self {
        let credentials = parameters.credentials.unwrap();
        Nash {
            transport: Client::from_key_data(
                &credentials.secret,
                &credentials.session,
                None,
                parameters.client_id,
                parameters.environment,
                parameters.timeout,
            )
            .await
            .unwrap(),
            exchange_info: ExchangeInfo::new(),
        }
    }

    fn inner_client(&self) -> Option<&Self::InnerClient> {
        Some(&self.transport)
    }
}

#[async_trait]
impl ExchangeMarketData for Nash {
    async fn get_historic_rates(&self, req: &GetHistoricRatesRequest) -> Result<Vec<Candle>> {
        let req: nash_protocol::protocol::list_candles::ListCandlesRequest = req.into();

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
        self.transport.run(req).await?;
        Ok(vec![])
    }

    async fn cancel_order(&self, req: &CancelOrderRequest) -> Result<OrderCanceled> {
        let req: nash_protocol::protocol::cancel_order::CancelOrderRequest = req.into();
        let resp = self.transport.run(req).await;
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
            let free = Decimal::from_str(&format!(
                "{}",
                resp.state_channel.get(asset).unwrap().amount.value
            ))
            .unwrap();
            let in_orders = Decimal::from_str(&format!(
                "{}",
                resp.in_orders.get(asset).unwrap().amount.value
            ))
            .unwrap();
            let total = free + in_orders;
            balances.push(Balance {
                asset: asset.name().to_string(),
                total,
                free,
            });
        }

        Ok(balances)
    }

    async fn get_all_open_orders(&self) -> Result<Vec<Order>> {
        let err = MissingImplementationContent {
            message: String::from("Not supported yet, market paramater is mandatory."),
        };
        Err(OpenLimitError::MissingImplementation(err))
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

        let resp = self.transport.run(req).await;

        Ok(
            Nash::unwrap_response::<nash_protocol::protocol::place_order::LimitOrderResponse>(
                resp,
            )?
            .into(),
        )
    }

    async fn limit_sell(&self, req: &OpenLimitOrderRequest) -> Result<Order> {
        let req: nash_protocol::protocol::place_order::LimitOrderRequest =
            Nash::convert_limit_order(req, nash_protocol::types::BuyOrSell::Sell);
        let resp = self.transport.run(req).await;

        Ok(
            Nash::unwrap_response::<nash_protocol::protocol::place_order::LimitOrderResponse>(
                resp,
            )?
            .into(),
        )
    }

    async fn market_sell(&self, _req: &OpenMarketOrderRequest) -> Result<Order> {
        let err = MissingImplementationContent {
            message: String::from("Not supported order type"),
        };
        Err(OpenLimitError::MissingImplementation(err))
    }

    async fn market_buy(&self, _req: &OpenMarketOrderRequest) -> Result<Order> {
        let err = MissingImplementationContent {
            message: String::from("Not supported order type"),
        };
        Err(OpenLimitError::MissingImplementation(err))
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
                .map_err(OpenLimitError::NashProtocolError),
            Err(err) => Err(OpenLimitError::NashProtocolError(err)),
        }
    }

    pub fn convert_limit_order(
        req: &OpenLimitOrderRequest,
        buy_or_sell: nash_protocol::types::BuyOrSell,
    ) -> nash_protocol::protocol::place_order::LimitOrderRequest {
        let market = nash_protocol::types::Market::from_str(&req.market_pair).unwrap();
        nash_protocol::protocol::place_order::LimitOrderRequest {
            cancellation_policy: nash_protocol::types::OrderCancellationPolicy::from(
                req.time_in_force,
            ),
            allow_taker: true,
            market,
            buy_or_sell,
            amount: format!("{}", req.size),
            price: format!("{}", req.price),
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
            last_update_id: None,
            bids: book.bids.into_iter().map(Into::into).collect(),
            asks: book.asks.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<nash_protocol::types::OrderbookOrder> for AskBid {
    fn from(resp: nash_protocol::types::OrderbookOrder) -> Self {
        let price = Decimal::from_str(&resp.price).unwrap();
        let qty = Decimal::from_str(&resp.amount.amount.value.to_string()).unwrap();
        Self { price, qty }
    }
}

impl From<&CancelOrderRequest> for nash_protocol::protocol::cancel_order::CancelOrderRequest {
    fn from(req: &CancelOrderRequest) -> Self {
        // TODO: why this param?
        let market = req.market_pair.clone().unwrap();

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

impl From<nash_protocol::protocol::place_order::LimitOrderResponse> for Order {
    fn from(resp: nash_protocol::protocol::place_order::LimitOrderResponse) -> Self {
        Self {
            id: resp.order_id,
            client_order_id: None,
            created_at: Some(resp.placed_at.timestamp_millis() as u64),
            market_pair: resp.market_name,
            order_type: resp.order_type.into(),
            size: Decimal::from(0),
            price: None,
            side: resp.buy_or_sell.into(),
            status: resp.status.into(),
        }
    }
}

impl TryFrom<&TradeHistoryRequest>
    for nash_protocol::protocol::list_account_trades::ListAccountTradesRequest
{
    type Error = OpenLimitError;
    fn try_from(req: &TradeHistoryRequest) -> crate::shared::Result<Self> {
        let (before, limit) = match req.paginator.clone() {
            Some(paginator) => (
                paginator.before,
                paginator.limit.map(|v| i64::try_from(v).unwrap()),
            ),
            None => (None, None),
        };

        // TODO: why is this required for Nash?
        let market = req
            .market_pair
            .clone()
            .expect("Market pair is required for Nash");
        let range: Option<nash_protocol::types::DateTimeRange> =
            req.paginator.clone().map(|paginator| paginator.into());

        Ok(Self {
            market,
            before,
            limit,
            range,
        })
    }
}

impl From<nash_protocol::types::Trade> for Trade {
    fn from(resp: nash_protocol::types::Trade) -> Self {
        let qty = Decimal::from_str(&format!("{}", &resp.amount.amount.value)).unwrap();
        let price = Decimal::from_str(&format!("{}", &resp.limit_price.amount.value)).unwrap();

        let (fees, order_id) = match resp.account_side {
            nash_protocol::types::AccountTradeSide::Taker => (
                Decimal::from_str(&format!("{}", &resp.taker_fee.amount.value)).unwrap(),
                resp.taker_order_id,
            ),
            _ => (Decimal::from(0), resp.maker_order_id),
        };

        Self {
            id: resp.id,
            created_at: resp.executed_at.timestamp_millis() as u64,
            fees: Some(fees),
            liquidity: Some(resp.account_side.into()),
            market_pair: resp.market.market_name(),
            order_id,
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

impl From<Paginator> for nash_protocol::types::DateTimeRange {
    fn from(paginator: Paginator) -> Self {
        Self {
            start: paginator.start_time.map(timestamp_to_utc_datetime).unwrap(),
            stop: paginator.end_time.map(timestamp_to_utc_datetime).unwrap(),
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

impl From<&GetHistoricRatesRequest> for nash_protocol::protocol::list_candles::ListCandlesRequest {
    fn from(req: &GetHistoricRatesRequest) -> Self {
        let market = req.market_pair.clone();

        let (before, limit) = match req.paginator.clone() {
            Some(p) => (p.before, p.limit.map(|v| i64::try_from(v).unwrap())),
            _ => (None, None),
        };

        Self {
            market,
            chronological: None,
            before,
            interval: Some(req.interval.try_into().unwrap()),
            limit,
            range: req.paginator.clone().map(Into::into),
        }
    }
}

fn try_split_paginator(paginator: Option<Paginator>) -> (Option<String>, Option<i64>) {
    match paginator {
        Some(paginator) => (
            paginator.before,
            paginator.limit.map(|v| i64::try_from(v).unwrap()),
        ),
        None => (None, None),
    }
}

impl TryFrom<&GetHistoricTradesRequest>
    for nash_protocol::protocol::list_trades::ListTradesRequest
{
    type Error = OpenLimitError;
    fn try_from(req: &GetHistoricTradesRequest) -> crate::shared::Result<Self> {
        let market = req.market_pair.clone();
        let (before, limit) = try_split_paginator(req.paginator.clone());
        //FIXME: Some issues with the graphql protocol for the market to be non nil
        Ok(Self {
            market,
            before,
            limit,
        })
    }
}

impl TryFrom<Interval> for nash_protocol::types::CandleInterval {
    type Error = OpenLimitError;
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
                Err(OpenLimitError::MissingImplementation(err))
            }
        }
    }
}

impl From<nash_protocol::types::Candle> for Candle {
    fn from(candle: nash_protocol::types::Candle) -> Self {
        let close = Decimal::from_str(&format!("{}", &candle.close_price.amount.value)).unwrap();
        let high = Decimal::from_str(&format!("{}", &candle.high_price.amount.value)).unwrap();
        let low = Decimal::from_str(&format!("{}", &candle.low_price.amount.value)).unwrap();
        let open = Decimal::from_str(&format!("{}", &candle.open_price.amount.value)).unwrap();
        let volume = Decimal::from_str(&format!("{}", &candle.a_volume.amount.value)).unwrap();

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
    type Error = OpenLimitError;
    fn try_from(req: &GetOrderHistoryRequest) -> crate::shared::Result<Self> {
        // TODO: why is this required for Nash
        let market = req
            .market_pair
            .clone()
            .expect("Market pair required for Nash");
        let (before, limit) = try_split_paginator(req.paginator.clone());
        let range: Option<nash_protocol::types::DateTimeRange> =
            req.paginator.clone().map(Into::into);

        Ok(Self {
            market,
            before,
            limit,
            range,
            buy_or_sell: None,
            order_type: None,
            status: None,
        })
    }
}

impl From<nash_protocol::types::Order> for Order {
    fn from(order: nash_protocol::types::Order) -> Self {
        let size = Decimal::from_str(&format!("{}", &order.amount_placed.amount.value)).unwrap();

        Self {
            id: order.id,
            client_order_id: None,
            created_at: Some(order.placed_at.timestamp_millis() as u64),
            market_pair: order.market.market_name(),
            order_type: order.order_type.into(),
            price: order
                .limit_price
                .map(|p| Decimal::from_str(&format!("{}", &p.amount.value)).unwrap()),
            size,
            side: order.buy_or_sell.into(),
            status: order.status.into(),
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

impl From<&GetPriceTickerRequest> for nash_protocol::protocol::get_ticker::TickerRequest {
    fn from(req: &GetPriceTickerRequest) -> Self {
        let market = req.market_pair.clone();

        Self { market }
    }
}

impl From<nash_protocol::protocol::get_ticker::TickerResponse> for Ticker {
    fn from(resp: nash_protocol::protocol::get_ticker::TickerResponse) -> Self {
        let ask = Decimal::from_str(&format!("{}", &resp.best_ask_price.amount.value)).unwrap();
        let bid = Decimal::from_str(&format!("{}", &resp.best_bid_price.amount.value)).unwrap();
        let price = (ask + bid) / Decimal::from(2);

        Self { price }
    }
}

impl From<&GetOrderRequest> for nash_protocol::protocol::get_account_order::GetAccountOrderRequest {
    fn from(req: &GetOrderRequest) -> Self {
        Self {
            order_id: req.id.clone(),
        }
    }
}

use futures::stream::{Stream, StreamExt};
use nash_protocol::protocol::ResponseOrError;
use std::{pin::Pin, task::Context, task::Poll};

pub struct NashStream {
    pub client: Client,
}

impl NashStream {
    pub async fn public(client_id: u64, sandbox: bool, timeout: u64) -> Self {
        let environment = if sandbox {
            Environment::Sandbox
        } else {
            Environment::Production
        };
        NashStream {
            client: Client::new(None, client_id, None, environment, timeout)
                .await
                .unwrap(),
        }
    }

    pub async fn with_credential(
        secret: &str,
        session: &str,
        client_id: u64,
        sandbox: bool,
        timeout: u64,
    ) -> Self {
        let environment = if sandbox {
            Environment::Sandbox
        } else {
            Environment::Production
        };
        NashStream {
            client: Client::from_key_data(secret, session, None, client_id, environment, timeout)
                .await
                .unwrap(),
        }
    }
}

impl Stream for NashStream {
    type Item = std::result::Result<
        ResponseOrError<nash_protocol::protocol::subscriptions::SubscriptionResponse>,
        nash_protocol::errors::ProtocolError,
    >;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.client.poll_next_unpin(cx)
    }
}

#[async_trait]
impl ExchangeWs for NashStream {
    type InitParams = NashParameters;
    async fn new(params: Self::InitParams) -> Self {
        let credentials = params.credentials.unwrap();
        Self {
            client: Client::from_key_data(
                &credentials.secret,
                &credentials.session,
                None,
                params.client_id,
                params.environment,
                params.timeout,
            )
            .await
            .unwrap(),
        }
    }
    async fn subscribe(&mut self, subscription: Subscription) -> Result<()> {
        let sub: nash_protocol::protocol::subscriptions::SubscriptionRequest = subscription.into();
        let _stream = Client::subscribe_protocol(&self.client, sub).await.unwrap();

        Ok(())
    }

    fn parse_message(&self, message: Self::Item) -> Result<OpenLimitsWebsocketMessage> {
        Ok(message.unwrap().consume_response().unwrap().into())
    }
}

impl From<Subscription> for nash_protocol::protocol::subscriptions::SubscriptionRequest {
    fn from(sub: Subscription) -> Self {
        match sub {
            Subscription::OrderBook(market, _depth) => Self::Orderbook(
                nash_protocol::protocol::subscriptions::updated_orderbook::SubscribeOrderbook {
                    market,
                },
            ),
            Subscription::Trade(market) => Self::Trades(
                nash_protocol::protocol::subscriptions::trades::SubscribeTrades { market },
            ),
            _ => panic!("Not supported Subscription"),
        }
    }
}

impl From<nash_protocol::protocol::subscriptions::SubscriptionResponse>
    for OpenLimitsWebsocketMessage
{
    fn from(message: nash_protocol::protocol::subscriptions::SubscriptionResponse) -> Self {
        match message {
            nash_protocol::protocol::subscriptions::SubscriptionResponse::Orderbook(resp) => {
                OpenLimitsWebsocketMessage::OrderBook(OrderBookResponse {
                    asks: resp.asks.into_iter().map(Into::into).collect(),
                    bids: resp.bids.into_iter().map(Into::into).collect(),
                    last_update_id: None,
                })
            }
            nash_protocol::protocol::subscriptions::SubscriptionResponse::Trades(resp) => {
                let trades = resp.trades.into_iter().map(|x| x.into()).collect();
                OpenLimitsWebsocketMessage::Trades(trades)
            }
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
                nash_protocol::types::OrderCancellationPolicy::GoodTilTime(expire_time.clone())
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
            Err(OpenLimitError::NashProtocolError(
                // FIXME: handle this better in both nash protocol and openlimits
                nash_protocol::errors::ProtocolError::coerce_static_from_str(&format!(
                    "{:#?}",
                    err
                )),
            ))
        } else {
            Ok(response.consume_response().unwrap()) // safe unwrap
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
