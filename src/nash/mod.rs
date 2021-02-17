use crate::{
    errors::OpenLimitsError,
    exchange::Exchange,
    exchange::ExchangeAccount,
    exchange::ExchangeMarketData,
    exchange_info::ExchangeInfo,
    exchange_info::MarketPairHandle,
    exchange_info::{ExchangeInfoRetrieval, MarketPair},
    exchange_ws::ExchangeWs,
    exchange_ws::Subscriptions,
    model::{
        Balance, CancelAllOrdersRequest, CancelOrderRequest, Candle,
        GetHistoricRatesRequest, GetHistoricTradesRequest, GetOrderHistoryRequest, GetOrderRequest,
        GetPriceTickerRequest, OpenLimitOrderRequest, OpenMarketOrderRequest,
        Order, OrderBookRequest, OrderBookResponse, OrderCanceled,
        Paginator, Ticker, Trade, TradeHistoryRequest,
    },
    shared::Result,
};
use async_trait::async_trait;
pub use nash_native_client::{Client, Environment};
use rust_decimal::prelude::*;
use std::convert::TryInto;

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
                params.client_id,
                params.environment,
                params.timeout,
            )
            .await?
        }
        None => {
            Client::from_keys_path(
                None,
                params.client_id,
                None,
                params.environment,
                params.timeout,
            )
            .await?
        }
    };

    if let Some(interval) = params.sign_states_loop_interval {
        client.start_background_state_signing(interval);
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

use futures::stream::{BoxStream, SelectAll, Stream, StreamExt};
use nash_protocol::protocol::{
    subscriptions::SubscriptionRequest,
    ResponseOrError,
};
use std::{pin::Pin, task::Context, task::Poll};
use tokio::time::Duration;
use nash_protocol::types::exchange::convert::SubscriptionResponseWrapper;

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
                    Err(OpenLimitsError::NotParsableResponse(f).into())
                }
            },
            Err(_) => Err(OpenLimitsError::SocketError().into()),
        });

        Ok(s.boxed())
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
