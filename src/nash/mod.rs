use async_trait::async_trait;
use nash_protocol::ws_client::client::Client;
use std::convert::TryFrom;

use crate::{
    errors::OpenLimitError,
    exchange::Exchange,
    model::{
        AskBid, Balance, CancelOrderRequest, OpenLimitOrderRequest, OrderBookRequest,
        OrderBookResponse, OrderCanceled, Paginator, TradeHistoryRequest, Trade
    },
    shared::Result,
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
        req: &CancelAllOrdersRequest<Self::PaginationType>,
    ) -> Result<Vec<OrderCanceled<Self::OrderIdType>>> {
        let req: nash_protocol::protocol::cancel_all_orders::types::CancelAllOrders = req.into();
        let resp = self.transport.run(req).await;
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
        paginator: Option<&Paginator<Self::PaginationType>>,
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
    // Wait for market not being optional
    // async fn get_all_open_orders(&mut self) -> Result<Vec<Order<Self::OrderIdType>>> {
    //     let req =
    //     nash_protocol::protocol::list_account_orders::types::ListAccountOrdersRequest{market: None, before: None, limit: None, buy_or_sell: None, order_type: None, status: None};
    // }

    // missing listCandles
    // async fn get_historic_rates(&mut self, req: &GetHistoricRatesRequest) -> Result<Vec<Candle>> {

    // }

    // Wait for market not being optional
    // missing listAccountOrders
    // async fn get_order_history(
    //         &mut self,
    //         req: &GetOrderHistoryRequest,
    //     ) -> Result<Vec<Order<Self::OrderIdType>>> {
    // }

    async fn get_trade_history(
        &mut self,
        req: &TradeHistoryRequest<Self::OrderIdType>,
    ) -> Result<Vec<Trade<Self::TradeIdType, Self::OrderIdType>>> {
        let req: nash_protocol::protocol::list_account_trades::types::ListAccountTradesRequest =
            req.into();
        
        let resp = self.transport.run(req).await;

        let resp: nash_protocol::protocol::list_account_trades::types::ListAccountTradesRequest =
            Nash::unwrap_response::<
                nash_protocol::protocol::list_account_trades::types::ListAccountTradesRequest,
            >(resp)?;
        Ok(resp.into())
    }

    // PENDING UPDATE ON OPEN ORDER RESP
    // async fn limit_buy(&mut self, req: &OpenLimitOrderRequest) -> Result<Order<Self::OrderIdType>> {
    //     let req: nash_protocol::protocol::place_order::types::LimitOrderRequest =  Nash::convert_limit_order(req, nash_protocol::types::BuyOrSell::Buy);

    //     let resp = self.transport.run(req).await;
    //     Ok(Nash::unwrap_response::<nash_protocol::protocol::place_order::types::LimitOrderResponse>(resp).into())
    // }

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
        req: OpenLimitOrderRequest,
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
        let market =
            nash_protocol::types::request::Market::from_str(&req.market_pair.unwrap()).unwrap();

        Self {
            market,
            order_id: req.id,
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

// impl From<CancelAllOrdersRequest> for  nash_protocol::protocol::cancel_all_orders::types::CancelAllOrders {
//     fn from(req: CancelAllOrdersRequest) -> Self {
//         let market = nash_protocol::types::request::Market::from_str(&req.market_pair).unwrap();
//         Self { market }
//     }
// }

// impl From<nash_protocol::protocol::place_order::types::LimitOrderResponse> for Order<String> {
//     fn from(resp: nash_protocol::protocol::place_order::types::LimitOrderResponse) -> Self {
//         Self {
//             created_at: None,
//             client_order_id: None,
//             id: resp.order_id,
//             market_pair: None,
//         }
//     }
// }

impl TryFrom<&TradeHistoryRequest<String, String>>
    for nash_protocol::protocol::list_account_trades::types::ListAccountTradesRequest
{
    fn try_from(req: &TradeHistoryRequest<String, String>) -> Result<Self> {
         let (before, limit) = match req.paginator {
            Some(paginator) => (paginator.before, paginator.limit),
            None => (None, None),
        };

        Ok(Self { market: req.market_pair, before, limit})
    }
}
