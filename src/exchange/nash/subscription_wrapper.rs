use nash_protocol::protocol::subscriptions::{SubscriptionRequest, SubscriptionResponse};
use openlimits_exchange::model::websocket::Subscription;
use nash_protocol::protocol::subscriptions::trades::SubscribeTrades;
use nash_protocol::protocol::subscriptions::updated_orderbook::SubscribeOrderbook;
use nash_protocol::types::market_pair::MarketPair;
use crate::model::websocket::{WebSocketResponse, OpenLimitsWebSocketMessage};
use std::convert::TryFrom;
use crate::errors::*;

#[derive(Clone, Debug)]
pub struct SubscriptionRequestWrapper(pub SubscriptionRequest);

#[derive(Clone, Debug)]
pub struct SubscriptionResponseWrapper(pub SubscriptionResponse);

impl TryFrom<Subscription> for SubscriptionRequestWrapper {
    type Error = OpenLimitsError;
    fn try_from(from: Subscription) -> Result<Self> {
        Ok(match from {
            Subscription::Trades(market) => {
                let market = MarketPair::from(market).0;
                let request = SubscriptionRequest::Trades(SubscribeTrades { market });
                Self(request)
            },
            Subscription::OrderBookUpdates(market) => {
                let market = MarketPair::from(market).0;
                let request = SubscriptionRequest::Orderbook(SubscribeOrderbook { market });
                Self(request)
            }
        })
    }
}
//
// impl<T> TryFrom<WebSocketResponse<T>> for SubscriptionResponseWrapper {
//     type Error = OpenLimitsError;
//     fn try_from(from: WebSocketResponse<T>) -> Result<Self> {
//         match from {
//             WebSocketResponse::Generic(message) => {
//                 OpenLimitsWebSocketMessage::Trades(trades) => {
//
//                 },
//                 OpenLimitsWebSocketMessage::OrderBook(orderbook) => {
//
//                 },
//                 OpenLimitsWebSocketMessage::OrderBookDiff(orderbook_diff) => {
//
//                 },
//                 OpenLimitsWebSocketMessage::Ping => {
//
//                 }
//             },
//             WebSocketResponse::Raw()
//         }
//         let response = SubscriptionResponse::Orderbook()
//         Self(response)
//     }
// }