//! This module provides functionality for communicating with the nash API.


mod nash_credentials;
mod nash_parameters;
mod nash_stream;
mod utils;

pub use nash_credentials::NashCredentials;
pub use nash_parameters::NashParameters;
pub use nash_stream::NashWebsocket;
pub use utils::client_from_params_failable;
pub use openlimits_exchange::shared;

use std::convert::TryInto;
use async_trait::async_trait;
use nash_native_client::Client;
use rust_decimal::prelude::*;
use openlimits_exchange::{
    errors::OpenLimitsError,
    model::{
        Balance, CancelAllOrdersRequest, CancelOrderRequest, Candle, GetHistoricRatesRequest,
        GetHistoricTradesRequest, GetOrderHistoryRequest, GetOrderRequest, GetPriceTickerRequest,
        OpenLimitOrderRequest, OpenMarketOrderRequest, Order,
        OrderBookRequest, OrderBookResponse, OrderCanceled, Paginator,
        Ticker, Trade, TradeHistoryRequest,
    },
};
use openlimits_exchange::shared::Result;
use openlimits_exchange::traits::info::ExchangeInfo;
use openlimits_exchange::traits::info::ExchangeInfoRetrieval;
use openlimits_exchange::traits::Exchange;
use openlimits_exchange::traits::ExchangeMarketData;
use openlimits_exchange::traits::ExchangeAccount;
use openlimits_exchange::traits::info::MarketPairInfo;
use openlimits_exchange::traits::info::MarketPairHandle;
use openlimits_exchange::model::market_pair::MarketPair;
use openlimits_exchange::MissingImplementationContent;

/// This struct is the main struct of this module and it is used for communications with the nash openlimits-exchange
pub struct Nash {
    pub transport: Client,
    pub exchange_info: ExchangeInfo,
}

#[async_trait]
impl Exchange for Nash {
    type InitParams = NashParameters;
    type InnerClient = Client;

    async fn new(params: Self::InitParams) -> Result<Self> {
        let nash = Self {
            exchange_info: ExchangeInfo::new(),
            transport: client_from_params_failable(params).await?,
        };
        nash.refresh_market_info().await.ok();
        Ok(nash)
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
        println!("{:#?}", req);
        let resp = self.transport.run_http(req).await;
        Ok(
            Nash::unwrap_response::<nash_protocol::protocol::place_order::PlaceOrderResponse>(
                resp,
            )?
            .into(),
        )
    }

    async fn market_buy(&self, _req: &OpenMarketOrderRequest) -> Result<Order> {
        let message = "Nash client doesn't implement market_buy".into();
        Err(OpenLimitsError::MissingImplementation(MissingImplementationContent { message }))
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
                .map_err(|error| OpenLimitsError::Generic(Box::new(error))),
            Err(error) => Err(OpenLimitsError::Generic(Box::new(error))),
        }
    }

    pub fn convert_limit_order(
        req: &OpenLimitOrderRequest,
        buy_or_sell: nash_protocol::types::BuyOrSell,
    ) -> nash_protocol::protocol::place_order::LimitOrderRequest {
        let market = req.market_pair.clone();
        let market = nash_protocol::types::market_pair::MarketPair::from(market).0;
        nash_protocol::protocol::place_order::LimitOrderRequest {
            client_order_id: req.client_order_id.clone(),
            cancellation_policy: req.time_in_force.into(),
            allow_taker: !req.post_only,
            market,
            buy_or_sell,
            amount: format!("{}", req.size),
            price: format!("{}", req.price),
        }
    }

    pub fn convert_market_request(
        req: &OpenMarketOrderRequest,
    ) -> nash_protocol::protocol::place_order::MarketOrderRequest {
        let market = req.market_pair.clone();
        let market = nash_protocol::types::market_pair::MarketPair::from(market).0;
        nash_protocol::protocol::place_order::MarketOrderRequest {
            client_order_id: req.client_order_id.clone(),
            market,
            amount: format!("{}", req.size),
        }
    }

    async fn list_markets(
        &self,
    ) -> Result<nash_protocol::protocol::list_markets::ListMarketsResponse> {
        let response = self
            .transport
            .run(nash_protocol::protocol::list_markets::ListMarketsRequest)
            .await?;
        if let Some(err) = response.error() {
            Err(OpenLimitsError::Generic(Box::new(err.clone())))
        } else {
            Ok(response
                .consume_response()
                .expect("Couldn't consume response.")) // safe unwrap
        }
    }
}

#[async_trait]
impl ExchangeInfoRetrieval for Nash {
    async fn retrieve_pairs(&self) -> Result<Vec<MarketPairInfo>> {
        Ok(self
            .list_markets()
            .await?
            .markets
            .iter()
            .map(|(symbol, v)| MarketPairInfo {
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

    async fn get_pair(&self, name: &MarketPair) -> Result<MarketPairHandle> {
        let name = nash_protocol::types::market_pair::MarketPair::from(name.clone()).0;
        self.exchange_info.get_pair(&name)
    }
}
