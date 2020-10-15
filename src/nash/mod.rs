use async_trait::async_trait;
use nash_native_client::ws_client::client::{Client, Environment};
use std::convert::{TryFrom, TryInto};

use crate::{
    errors::{MissingImplementationContent, OpenLimitError},
    exchange::Exchange,
    exchange::ExchangeInstantiation,
    exchange_info::ExchangeInfo,
    exchange_info::MarketPair,
    exchange_info::{ExchangeInfoRetrieval, MarketPairHandle},
    exchange_ws::ExchangeWs,
    model::{
        websocket::{OpenLimitsWebsocketMessage, Subscription},
        AskBid, Balance, CancelAllOrdersRequest, CancelOrderRequest, Candle,
        GetHistoricRatesRequest, GetHistoricTradesRequest, GetOrderHistoryRequest, GetOrderRequest,
        GetPriceTickerRequest, Interval, Liquidity, OpenLimitOrderRequest, OpenMarketOrderRequest,
        Order, OrderBookRequest, OrderBookResponse, OrderCanceled, OrderStatus, Paginator, Side,
        Ticker, Trade, TradeHistoryRequest,
    },
    shared::{timestamp_to_utc_datetime, Result},
};
use rust_decimal::prelude::*;

pub struct Nash {
    transport: Client,
    exchange_info: ExchangeInfo,
}

impl Nash {
    pub async fn public(client_id: u64, sandbox: bool, timeout: u64) -> Self {
        let environment = if sandbox {
            Environment::Sandbox
        } else {
            Environment::Production
        };
        Nash {
            transport: Client::new(None, client_id, None, environment, timeout)
                .await
                .unwrap(),
            exchange_info: ExchangeInfo::new(),
        }
    }

    pub async fn with_credential(
        secret: &str,
        session: &str,
        client_id: u64,
        environment: Environment,
        timeout: u64,
    ) -> Self {
        ExchangeInstantiation::new(NashParameters {
            credentials: Some(NashCredentials {
                secret: secret.to_string(),
                session: session.to_string(),
            }),
            client_id,
            environment,
            timeout,
        })
        .await
    }
}

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

#[async_trait]
impl ExchangeInstantiation for Nash {
    type Parameters = NashParameters;

    async fn new(parameters: Self::Parameters) -> Self {
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
}

#[async_trait]
impl Exchange for Nash {
    type OrderIdType = String;
    type TradeIdType = String;
    type PaginationType = String;

    async fn cancel_all_orders(
        &self,
        req: &CancelAllOrdersRequest,
    ) -> Result<Vec<OrderCanceled<Self::OrderIdType>>> {
        let req: nash_protocol::protocol::cancel_all_orders::CancelAllOrders = req.into();
        self.transport.run(req).await?;
        Ok(vec![])
    }

    async fn cancel_order(
        &self,
        req: &CancelOrderRequest<Self::OrderIdType>,
    ) -> Result<OrderCanceled<Self::OrderIdType>> {
        let req: nash_protocol::protocol::cancel_order::CancelOrderRequest = req.into();
        let resp = self.transport.run(req).await;
        Ok(
            Nash::unwrap_response::<nash_protocol::protocol::cancel_order::CancelOrderResponse>(
                resp,
            )?
            .into(),
        )
    }

    async fn get_account_balances(
        &self,
        _paginator: Option<&Paginator<Self::PaginationType>>,
    ) -> Result<Vec<Balance>> {
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

    async fn get_all_open_orders(&self) -> Result<Vec<Order<Self::OrderIdType>>> {
        let err = MissingImplementationContent {
            message: String::from("Not supported yet, market paramater is mandatory."),
        };
        Err(OpenLimitError::MissingImplementation(err))
    }

    async fn get_historic_rates(
        &self,
        req: &GetHistoricRatesRequest<Self::PaginationType>,
    ) -> Result<Vec<Candle>> {
        let req: nash_protocol::protocol::list_candles::ListCandlesRequest = req.into();

        let resp = self.transport.run(req).await;

        let resp: nash_protocol::protocol::list_candles::ListCandlesResponse =
            Nash::unwrap_response::<nash_protocol::protocol::list_candles::ListCandlesResponse>(
                resp,
            )?;

        Ok(resp.candles.into_iter().map(Into::into).collect())
    }

    async fn get_historic_trades(
        &self,
        req: &GetHistoricTradesRequest<Self::PaginationType>,
    ) -> Result<Vec<Trade<Self::TradeIdType, Self::OrderIdType>>> {
        let req: nash_protocol::protocol::list_trades::ListTradesRequest = req.try_into()?;
        let resp = self.transport.run(req).await;

        let resp: nash_protocol::protocol::list_trades::ListTradesResponse = Nash::unwrap_response::<
            nash_protocol::protocol::list_trades::ListTradesResponse,
        >(resp)?;

        Ok(resp.trades.into_iter().map(Into::into).collect())
    }

    async fn get_order_history(
        &self,
        req: &GetOrderHistoryRequest<Self::PaginationType>,
    ) -> Result<Vec<Order<Self::OrderIdType>>> {
        let req: nash_protocol::protocol::list_account_orders::ListAccountOrdersRequest =
            req.try_into()?;

        let resp = self.transport.run(req).await;

        let resp: nash_protocol::protocol::list_account_orders::ListAccountOrdersResponse =
            Nash::unwrap_response::<
                nash_protocol::protocol::list_account_orders::ListAccountOrdersResponse,
            >(resp)?;

        Ok(resp.orders.into_iter().map(Into::into).collect())
    }

    async fn get_trade_history(
        &self,
        req: &TradeHistoryRequest<Self::OrderIdType, Self::PaginationType>,
    ) -> Result<Vec<Trade<Self::TradeIdType, Self::OrderIdType>>> {
        let req: nash_protocol::protocol::list_account_trades::ListAccountTradesRequest =
            req.try_into()?;

        let resp = self.transport.run(req).await;

        let resp: nash_protocol::protocol::list_account_trades::ListAccountTradesResponse =
            Nash::unwrap_response::<
                nash_protocol::protocol::list_account_trades::ListAccountTradesResponse,
            >(resp)?;

        Ok(resp.trades.into_iter().map(Into::into).collect())
    }

    async fn limit_buy(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self::OrderIdType>> {
        let req: nash_protocol::protocol::place_order::LimitOrderRequest =
            Nash::convert_limit_order(req, nash_protocol::types::BuyOrSell::Buy);

        let resp = self.transport.run(req).await;
        println!("{:?}", resp);

        Ok(
            Nash::unwrap_response::<nash_protocol::protocol::place_order::LimitOrderResponse>(
                resp,
            )?
            .into(),
        )
    }

    async fn limit_sell(&self, req: &OpenLimitOrderRequest) -> Result<Order<Self::OrderIdType>> {
        let req: nash_protocol::protocol::place_order::LimitOrderRequest =
            Nash::convert_limit_order(req, nash_protocol::types::BuyOrSell::Sell);

        let resp = self.transport.run(req).await;
        println!("{:?}", resp);

        Ok(
            Nash::unwrap_response::<nash_protocol::protocol::place_order::LimitOrderResponse>(
                resp,
            )?
            .into(),
        )
    }

    async fn market_sell(&self, _req: &OpenMarketOrderRequest) -> Result<Order<Self::OrderIdType>> {
        let err = MissingImplementationContent {
            message: String::from("Not supported order type"),
        };
        Err(OpenLimitError::MissingImplementation(err))
    }

    async fn market_buy(&self, _req: &OpenMarketOrderRequest) -> Result<Order<Self::OrderIdType>> {
        let err = MissingImplementationContent {
            message: String::from("Not supported order type"),
        };
        Err(OpenLimitError::MissingImplementation(err))
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

    async fn get_order(
        &self,
        req: &GetOrderRequest<Self::OrderIdType>,
    ) -> Result<Order<Self::OrderIdType>> {
        let req: nash_protocol::protocol::get_account_order::GetAccountOrderRequest = req.into();
        let resp = self.transport.run(req).await;
        let resp = Nash::unwrap_response::<
            nash_protocol::protocol::get_account_order::GetAccountOrderResponse,
        >(resp)?;
        Ok(resp.order.into())
    }

    async fn refresh_market_info(&self) -> Result<Vec<MarketPairHandle>> {
        self.exchange_info.refresh(self).await
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
            cancellation_policy: nash_protocol::types::OrderCancellationPolicy::GoodTilCancelled,
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
        let market = nash_protocol::types::Market::from_str(&req.market_pair).unwrap();
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

impl From<&CancelOrderRequest<String>>
    for nash_protocol::protocol::cancel_order::CancelOrderRequest
{
    fn from(req: &CancelOrderRequest<String>) -> Self {
        let pair = req.market_pair.clone().unwrap();
        let market = nash_protocol::types::Market::from_str(&pair).unwrap();

        Self {
            market,
            order_id: req.id.clone(),
        }
    }
}

impl From<nash_protocol::protocol::cancel_order::CancelOrderResponse> for OrderCanceled<String> {
    fn from(resp: nash_protocol::protocol::cancel_order::CancelOrderResponse) -> Self {
        Self { id: resp.order_id }
    }
}

impl From<&CancelAllOrdersRequest> for nash_protocol::protocol::cancel_all_orders::CancelAllOrders {
    fn from(req: &CancelAllOrdersRequest) -> Self {
        let pair = req.market_pair.clone().unwrap();
        let market = nash_protocol::types::Market::from_str(&pair).unwrap();
        Self { market }
    }
}

impl From<nash_protocol::protocol::place_order::LimitOrderResponse> for Order<String> {
    fn from(resp: nash_protocol::protocol::place_order::LimitOrderResponse) -> Self {
        Self {
            created_at: None,
            client_order_id: None,
            id: resp.order_id,
            market_pair: resp.market_name,
            order_type: resp.order_type.to_string(),
            price: None,
            size: Decimal::from(0),
            side: Side::Buy,
            status: resp.status.into(),
        }
    }
}

impl TryFrom<&TradeHistoryRequest<String, String>>
    for nash_protocol::protocol::list_account_trades::ListAccountTradesRequest
{
    type Error = OpenLimitError;
    fn try_from(req: &TradeHistoryRequest<String, String>) -> crate::shared::Result<Self> {
        let (before, limit) = match req.paginator.clone() {
            Some(paginator) => (
                paginator.before,
                paginator.limit.map(|v| i64::try_from(v).unwrap()),
            ),
            None => (None, None),
        };

        let market = match req.market_pair.clone() {
            Some(pair) => {
                let market = nash_protocol::types::Market::from_str(&pair)?;
                Some(market)
            }
            None => None,
        };
        let range: Option<nash_protocol::types::DateTimeRange> =
            req.paginator.clone().map(Into::into);

        //FIXME: Some issues with the graphql protocol for the market to be non nil
        let market = market.unwrap();
        Ok(Self {
            market,
            before,
            limit,
            range,
        })
    }
}

impl From<nash_protocol::types::Trade> for Trade<String, String> {
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

impl From<Paginator<String>> for nash_protocol::types::DateTimeRange {
    fn from(paginator: Paginator<String>) -> Self {
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

impl From<&GetHistoricRatesRequest<String>>
    for nash_protocol::protocol::list_candles::ListCandlesRequest
{
    fn from(req: &GetHistoricRatesRequest<String>) -> Self {
        let market = nash_protocol::types::Market::from_str(&req.market_pair).unwrap();

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

fn try_split_paginator(paginator: Option<Paginator<String>>) -> (Option<String>, Option<i64>) {
    match paginator {
        Some(paginator) => (
            paginator.before,
            paginator.limit.map(|v| i64::try_from(v).unwrap()),
        ),
        None => (None, None),
    }
}

impl TryFrom<&GetHistoricTradesRequest<String>>
    for nash_protocol::protocol::list_trades::ListTradesRequest
{
    type Error = OpenLimitError;
    fn try_from(req: &GetHistoricTradesRequest<String>) -> crate::shared::Result<Self> {
        let market = nash_protocol::types::Market::from_str(&req.market_pair)?;
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

impl TryFrom<&GetOrderHistoryRequest<String>>
    for nash_protocol::protocol::list_account_orders::ListAccountOrdersRequest
{
    type Error = OpenLimitError;
    fn try_from(req: &GetOrderHistoryRequest<String>) -> crate::shared::Result<Self> {
        let market = match req.clone().market_pair {
            Some(pair) => {
                let market = nash_protocol::types::Market::from_str(&pair)?;
                Some(market)
            }
            None => None,
        };
        let (before, limit) = try_split_paginator(req.paginator.clone());
        let range: Option<nash_protocol::types::DateTimeRange> =
            req.paginator.clone().map(Into::into);

        //FIXME: Some issues with the graphql protocol for the market to be non nil
        let market = market.unwrap();
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

impl From<nash_protocol::types::Order> for Order<String> {
    fn from(order: nash_protocol::types::Order) -> Self {
        let size = Decimal::from_str(&format!("{}", &order.amount_placed.amount.value)).unwrap();
        let order_type = match order.order_type {
            nash_protocol::types::OrderType::Limit => "limit",
            nash_protocol::types::OrderType::Market => "market",
            nash_protocol::types::OrderType::StopLimit => "stop_limit",
            nash_protocol::types::OrderType::StopMarket => "stop_market",
        };
        Self {
            id: order.id,
            client_order_id: None,
            created_at: Some(order.placed_at.timestamp_millis() as u64),
            market_pair: order.market.market_name(),
            order_type: String::from(order_type),
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
        let market = nash_protocol::types::Market::from_str(&req.market_pair).unwrap();

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

impl From<&GetOrderRequest<String>>
    for nash_protocol::protocol::get_account_order::GetAccountOrderRequest
{
    fn from(req: &GetOrderRequest<String>) -> Self {
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
            Subscription::OrderBook(symbol, _depth) => {
                let market = nash_protocol::types::Market::from_str(&symbol).unwrap();
                Self::Orderbook(
                    nash_protocol::protocol::subscriptions::updated_orderbook::SubscribeOrderbook {
                        market,
                    },
                )
            }
            Subscription::Trade(symbol) => {
                let market = nash_protocol::types::Market::from_str(&symbol).unwrap();
                Self::Trades(
                    nash_protocol::protocol::subscriptions::trades::SubscribeTrades { market },
                )
            }
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
                    asks: resp.asks.clone().into_iter().map(Into::into).collect(),
                    bids: resp.bids.clone().into_iter().map(Into::into).collect(),
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
}
