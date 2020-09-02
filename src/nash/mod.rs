use async_trait::async_trait;
use nash_native_client::ws_client::client::Client;
use std::convert::{TryFrom, TryInto};

use crate::{
    errors::{MissingImplementationContent, OpenLimitError},
    exchange::Exchange,
    model::{
        AskBid, Balance, CancelAllOrdersRequest, CancelOrderRequest, Candle,
        GetHistoricRatesRequest, GetOrderHistoryRequest, GetOrderRequest, GetPriceTickerRequest,
        Interval, Liquidity, OpenLimitOrderRequest, OpenMarketOrderRequest, Order,
        OrderBookRequest, OrderBookResponse, OrderCanceled, OrderStatus, Paginator, Side, Ticker,
        Trade, TradeHistoryRequest,
    },
    shared::{timestamp_to_utc_datetime, Result},
};
use rust_decimal::prelude::*;

pub struct Nash {
    transport: Client,
}

impl Nash {
    pub async fn with_credential(
        secret: &str,
        session: &str,
        client_id: u64,
        sandbox: bool,
        timeout: u64,
    ) -> Self {
        Nash {
            transport: Client::from_key_data(secret, session, client_id, sandbox, timeout)
                .await
                .unwrap(),
        }
    }
}

#[async_trait]
impl Exchange for Nash {
    type OrderIdType = String;
    type TradeIdType = String;
    type PaginationType = String;

    async fn cancel_all_orders(
        &mut self,
        req: &CancelAllOrdersRequest,
    ) -> Result<Vec<OrderCanceled<Self::OrderIdType>>> {
        let req: nash_protocol::protocol::cancel_all_orders::types::CancelAllOrders = req.into();
        self.transport.run(req).await?;
        Ok(vec![])
    }

    async fn cancel_order(
        &mut self,
        req: &CancelOrderRequest<Self::OrderIdType>,
    ) -> Result<OrderCanceled<Self::OrderIdType>> {
        let req: nash_protocol::protocol::cancel_order::types::CancelOrderRequest = req.into();
        let resp = self.transport.run(req).await;
        Ok(Nash::unwrap_response::<
            nash_protocol::protocol::cancel_order::types::CancelOrderResponse,
        >(resp)?
        .into())
    }

    async fn get_account_balances(
        &mut self,
        _paginator: Option<&Paginator<Self::PaginationType>>,
    ) -> Result<Vec<Balance>> {
        let req =
            nash_protocol::protocol::list_account_balances::types::ListAccountBalancesRequest {};
        let resp = self.transport.run(req).await;

        let resp:nash_protocol::protocol::list_account_balances::types::ListAccountBalancesResponse = Nash::unwrap_response::<nash_protocol::protocol::list_account_balances::types::ListAccountBalancesResponse>(resp)?;

        let balances: Vec<Balance> = resp
            .personal
            .iter()
            .map(|(asset, amount)| {
                let total = Decimal::from_str(&format!("{}", &amount.amount.value)).unwrap();
                Balance {
                    asset: String::from(asset.name()),
                    total: total,
                    free: total,
                }
            })
            .collect();
        Ok(balances)
    }

    async fn get_all_open_orders(&mut self) -> Result<Vec<Order<Self::OrderIdType>>> {
        let err = MissingImplementationContent {
            message: String::from("Not supported yet, market paramater is mandatory."),
        };
        Err(OpenLimitError::MissingImplementation(err))
    }

    async fn get_historic_rates(
        &mut self,
        req: &GetHistoricRatesRequest<Self::PaginationType>,
    ) -> Result<Vec<Candle>> {
        let req: nash_protocol::protocol::list_candles::types::ListCandlesRequest = req.into();

        let resp = self.transport.run(req).await;

        let resp: nash_protocol::protocol::list_candles::types::ListCandlesResponse =
            Nash::unwrap_response::<
                nash_protocol::protocol::list_candles::types::ListCandlesResponse,
            >(resp)?;

        Ok(resp.candles.into_iter().map(Into::into).collect())
    }

    async fn get_order_history(
        &mut self,
        req: &GetOrderHistoryRequest<Self::PaginationType>,
    ) -> Result<Vec<Order<Self::OrderIdType>>> {
        let req: nash_protocol::protocol::list_account_orders::types::ListAccountOrdersRequest =
            req.try_into()?;

        let resp = self.transport.run(req).await;

        let resp: nash_protocol::protocol::list_account_orders::types::ListAccountOrdersResponse =
            Nash::unwrap_response::<
                nash_protocol::protocol::list_account_orders::types::ListAccountOrdersResponse,
            >(resp)?;

        Ok(resp.orders.into_iter().map(Into::into).collect())
    }

    async fn get_trade_history(
        &mut self,
        req: &TradeHistoryRequest<Self::OrderIdType, Self::PaginationType>,
    ) -> Result<Vec<Trade<Self::TradeIdType, Self::OrderIdType>>> {
        let req: nash_protocol::protocol::list_account_trades::types::ListAccountTradesRequest =
            req.try_into()?;

        let resp = self.transport.run(req).await;

        let resp: nash_protocol::protocol::list_account_trades::types::ListAccountTradesResponse =
            Nash::unwrap_response::<
                nash_protocol::protocol::list_account_trades::types::ListAccountTradesResponse,
            >(resp)?;

        Ok(resp.trades.into_iter().map(Into::into).collect())
    }

    async fn limit_buy(&mut self, req: &OpenLimitOrderRequest) -> Result<Order<Self::OrderIdType>> {
        let req: nash_protocol::protocol::place_order::types::LimitOrderRequest =
            Nash::convert_limit_order(req, nash_protocol::types::BuyOrSell::Buy);

        let resp = self.transport.run(req).await;
        Ok(Nash::unwrap_response::<nash_protocol::protocol::place_order::types::LimitOrderResponse>(resp)?.into())
    }

    async fn limit_sell(
        &mut self,
        req: &OpenLimitOrderRequest,
    ) -> Result<Order<Self::OrderIdType>> {
        let req: nash_protocol::protocol::place_order::types::LimitOrderRequest =
            Nash::convert_limit_order(req, nash_protocol::types::BuyOrSell::Sell);

        let resp = self.transport.run(req).await;
        Ok(Nash::unwrap_response::<nash_protocol::protocol::place_order::types::LimitOrderResponse>(resp)?.into())
    }

    async fn market_sell(
        &mut self,
        _req: &OpenMarketOrderRequest,
    ) -> Result<Order<Self::OrderIdType>> {
        let err = MissingImplementationContent {
            message: String::from("Not supported order type"),
        };
        Err(OpenLimitError::MissingImplementation(err))
    }

    async fn market_buy(
        &mut self,
        _req: &OpenMarketOrderRequest,
    ) -> Result<Order<Self::OrderIdType>> {
        let err = MissingImplementationContent {
            message: String::from("Not supported order type"),
        };
        Err(OpenLimitError::MissingImplementation(err))
    }

    async fn get_price_ticker(&mut self, req: &GetPriceTickerRequest) -> Result<Ticker> {
        let req: nash_protocol::protocol::get_ticker::types::TickerRequest = req.into();
        let resp = self.transport.run(req).await;
        Ok(
            Nash::unwrap_response::<nash_protocol::protocol::get_ticker::types::TickerResponse>(
                resp,
            )?
            .into(),
        )
    }

    async fn order_book(&mut self, req: &OrderBookRequest) -> Result<OrderBookResponse> {
        let req: nash_protocol::protocol::orderbook::types::OrderbookRequest = req.into();
        let resp = self.transport.run(req).await;
        Ok(
            Nash::unwrap_response::<nash_protocol::protocol::orderbook::types::OrderbookResponse>(
                resp,
            )?
            .into(),
        )
    }

    async fn get_order(
        &mut self,
        req: &GetOrderRequest<Self::OrderIdType>,
    ) -> Result<Order<Self::OrderIdType>> {
        let req: nash_protocol::protocol::get_account_order::types::GetAccountOrderRequest =
            req.into();
        let resp = self.transport.run(req).await;
        let resp = Nash::unwrap_response::<
            nash_protocol::protocol::get_account_order::types::GetAccountOrderResponse,
        >(resp)?;
        Ok(resp.order.into())
    }

    async fn refresh_market_info(&mut self) -> Result<()> {
        println!("Nash doesn't support refreshing markets");
        Ok(())
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
                .map_err(|err| OpenLimitError::NashProtocolError(err)),
            Err(err) => Err(OpenLimitError::NashProtocolError(err)),
        }
    }

    pub fn convert_limit_order(
        req: &OpenLimitOrderRequest,
        buy_or_sell: nash_protocol::types::BuyOrSell,
    ) -> nash_protocol::protocol::place_order::types::LimitOrderRequest {
        let market = nash_protocol::types::request::Market::from_str(&req.market_pair).unwrap();

        nash_protocol::protocol::place_order::types::LimitOrderRequest {
            market,
            buy_or_sell,
            amount: format!("{}", req.size),
            price: format!("{}", req.price),
        }
    }
}
impl From<&OrderBookRequest> for nash_protocol::protocol::orderbook::types::OrderbookRequest {
    fn from(req: &OrderBookRequest) -> Self {
        let market = nash_protocol::types::request::Market::from_str(&req.market_pair).unwrap();
        Self { market }
    }
}

impl From<nash_protocol::protocol::orderbook::types::OrderbookResponse> for OrderBookResponse {
    fn from(book: nash_protocol::protocol::orderbook::types::OrderbookResponse) -> Self {
        Self {
            last_update_id: None,
            bids: book.bids.into_iter().map(Into::into).collect(),
            asks: book.asks.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<nash_protocol::protocol::orderbook::types::Order> for AskBid {
    fn from(resp: nash_protocol::protocol::orderbook::types::Order) -> Self {
        let price = Decimal::from_str(&resp.price).unwrap();
        let qty = Decimal::from_str(&format!("{}", resp.amount.amount.value)).unwrap();
        Self { price, qty }
    }
}

impl From<&CancelOrderRequest<String>>
    for nash_protocol::protocol::cancel_order::types::CancelOrderRequest
{
    fn from(req: &CancelOrderRequest<String>) -> Self {
        let pair = req.market_pair.clone().unwrap();
        let market = nash_protocol::types::request::Market::from_str(&pair).unwrap();

        Self {
            market,
            order_id: req.id.clone(),
        }
    }
}

impl From<nash_protocol::protocol::cancel_order::types::CancelOrderResponse>
    for OrderCanceled<String>
{
    fn from(resp: nash_protocol::protocol::cancel_order::types::CancelOrderResponse) -> Self {
        Self { id: resp.order_id }
    }
}

impl From<&CancelAllOrdersRequest>
    for nash_protocol::protocol::cancel_all_orders::types::CancelAllOrders
{
    fn from(req: &CancelAllOrdersRequest) -> Self {
        let pair = req.market_pair.clone().unwrap();
        let market = nash_protocol::types::request::Market::from_str(&pair).unwrap();
        Self { market }
    }
}

impl From<nash_protocol::protocol::place_order::types::LimitOrderResponse> for Order<String> {
    fn from(resp: nash_protocol::protocol::place_order::types::LimitOrderResponse) -> Self {
        Self {
            created_at: None,
            client_order_id: None,
            id: resp.order_id,
            market_pair: resp.market.market_name(),
            order_type: resp.order_type.to_string(),
            price: None,
            size: Decimal::from(0),
            side: Side::Buy,
            status: resp.status.into(),
        }
    }
}

impl TryFrom<&TradeHistoryRequest<String, String>>
    for nash_protocol::protocol::list_account_trades::types::ListAccountTradesRequest
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
                let market = nash_protocol::types::request::Market::from_str(&pair)?;
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
            fees,
            liquidity: Some(resp.account_side.into()),
            market_pair: resp.market.market_name(),
            order_id,
            price,
            qty,
            side: resp.direction.into(),
        }
    }
}

impl From<nash_protocol::types::request::BuyOrSell> for Side {
    fn from(side: nash_protocol::types::request::BuyOrSell) -> Self {
        match side {
            nash_protocol::types::request::BuyOrSell::Buy => Side::Buy,
            nash_protocol::types::request::BuyOrSell::Sell => Side::Sell,
        }
    }
}

impl From<Paginator<String>> for nash_protocol::types::request::DateTimeRange {
    fn from(paginator: Paginator<String>) -> Self {
        Self {
            start: paginator.start_time.map(timestamp_to_utc_datetime).unwrap(),
            stop: paginator.end_time.map(timestamp_to_utc_datetime).unwrap(),
        }
    }
}

impl From<nash_protocol::types::response::AccountTradeSide> for Liquidity {
    fn from(side: nash_protocol::types::response::AccountTradeSide) -> Self {
        match side {
            nash_protocol::types::AccountTradeSide::Taker => Liquidity::Taker,
            _ => Liquidity::Maker,
        }
    }
}

impl From<&GetHistoricRatesRequest<String>>
    for nash_protocol::protocol::list_candles::types::ListCandlesRequest
{
    fn from(req: &GetHistoricRatesRequest<String>) -> Self {
        let market = nash_protocol::types::request::Market::from_str(&req.market_pair).unwrap();

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

impl TryFrom<Interval> for nash_protocol::types::request::CandleInterval {
    type Error = OpenLimitError;
    fn try_from(interval: Interval) -> crate::shared::Result<Self> {
        match interval {
            Interval::OneMinute => Ok(nash_protocol::types::request::CandleInterval::OneMinute),
            Interval::FiveMinutes => Ok(nash_protocol::types::request::CandleInterval::FiveMinute),
            Interval::FifteenMinutes => {
                Ok(nash_protocol::types::request::CandleInterval::FifteenMinute)
            }
            Interval::ThirtyMinutes => {
                Ok(nash_protocol::types::request::CandleInterval::ThirtyMinute)
            }
            Interval::OneHour => Ok(nash_protocol::types::request::CandleInterval::OneHour),
            Interval::SixHours => Ok(nash_protocol::types::request::CandleInterval::SixHour),
            Interval::TwelveHours => Ok(nash_protocol::types::request::CandleInterval::TwelveHour),
            Interval::OneDay => Ok(nash_protocol::types::request::CandleInterval::OneDay),
            _ => {
                let err = MissingImplementationContent {
                    message: String::from("Not supported interval"),
                };
                Err(OpenLimitError::MissingImplementation(err))
            }
        }
    }
}

impl From<nash_protocol::types::request::Candle> for Candle {
    fn from(candle: nash_protocol::types::request::Candle) -> Self {
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
    for nash_protocol::protocol::list_account_orders::types::ListAccountOrdersRequest
{
    type Error = OpenLimitError;
    fn try_from(req: &GetOrderHistoryRequest<String>) -> crate::shared::Result<Self> {
        let (before, limit) = match req.clone().paginator {
            Some(paginator) => (
                paginator.before,
                paginator.limit.map(|v| i64::try_from(v).unwrap()),
            ),
            None => (None, None),
        };

        let market = match req.clone().market_pair {
            Some(pair) => {
                let market = nash_protocol::types::request::Market::from_str(&pair)?;
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
            buy_or_sell: None,
            order_type: None,
            status: None,
        })
    }
}

impl From<nash_protocol::types::response::Order> for Order<String> {
    fn from(order: nash_protocol::types::response::Order) -> Self {
        let size = Decimal::from_str(&format!("{}", &order.amount_placed.amount.value)).unwrap();
        let order_type = match order.order_type {
            nash_protocol::types::OrderType::Limit => "limit",
            nash_protocol::types::OrderType::Market => "market",
            nash_protocol::types::OrderType::StopLimit => "stop_limit",
            nash_protocol::types::OrderType::StopMarket => "stop_market",
        };
        Self {
            id: String::from("???"),
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
impl From<nash_protocol::types::response::OrderStatus> for OrderStatus {
    fn from(status: nash_protocol::types::response::OrderStatus) -> Self {
        match status {
            nash_protocol::types::response::OrderStatus::Filled => OrderStatus::Filled,
            nash_protocol::types::response::OrderStatus::Open => OrderStatus::Open,
            nash_protocol::types::response::OrderStatus::Canceled => OrderStatus::Canceled,
            nash_protocol::types::response::OrderStatus::Pending => OrderStatus::Pending,
        }
    }
}

impl From<&GetPriceTickerRequest> for nash_protocol::protocol::get_ticker::types::TickerRequest {
    fn from(req: &GetPriceTickerRequest) -> Self {
        let market = nash_protocol::types::request::Market::from_str(&req.market_pair).unwrap();

        Self { market }
    }
}

impl From<nash_protocol::protocol::get_ticker::types::TickerResponse> for Ticker {
    fn from(resp: nash_protocol::protocol::get_ticker::types::TickerResponse) -> Self {
        let ask = Decimal::from_str(&format!("{}", &resp.best_ask_price.amount.value)).unwrap();
        let bid = Decimal::from_str(&format!("{}", &resp.best_bid_price.amount.value)).unwrap();
        let price = (ask + bid) / Decimal::from(2);

        Self { price }
    }
}

impl From<&GetOrderRequest<String>>
    for nash_protocol::protocol::get_account_order::types::GetAccountOrderRequest
{
    fn from(req: &GetOrderRequest<String>) -> Self {
        Self {
            order_id: req.id.clone(),
        }
    }
}
