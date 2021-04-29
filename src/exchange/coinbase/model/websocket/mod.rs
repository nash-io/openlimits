use crate::errors::OpenLimitsError;
use crate::model::websocket::OpenLimitsWebSocketMessage;
use crate::model::websocket::WebSocketResponse;
use crate::model::AskBid;
use crate::model::OrderBookResponse;
use crate::shared::Result;
use std::convert::{TryFrom, TryInto};

use super::OrderSide;

mod activate;
mod auth;
mod change;
mod channel_type;
mod channel;
mod coinbase_subscription;
mod coinbase_websocket_message;
mod done;
mod full;
mod input_message;
mod level2_snapshot_record;
mod level2_update_record;
mod level2;
mod match_;
mod open;
mod reason;
mod received;
mod stop_type;
mod subscribe_cmd;
mod subscribe;
mod ticker;

pub use activate::Activate;
pub use auth::Auth;
pub use change::Change;
pub use channel_type::ChannelType;
pub use channel::Channel;
pub use coinbase_subscription::CoinbaseSubscription;
pub use coinbase_websocket_message::CoinbaseWebsocketMessage;
pub use done::Done;
pub use full::Full;
pub use input_message::InputMessage;
pub use level2_snapshot_record::Level2SnapshotRecord;
pub use level2_update_record::Level2UpdateRecord;
pub use level2::Level2;
pub use match_::Match;
pub use open::Open;
pub use reason::Reason;
pub use received::Received;
pub use stop_type::StopType;
pub use subscribe_cmd::SubscribeCmd;
pub use subscribe::Subscribe;
pub use ticker::Ticker;

impl TryFrom<CoinbaseWebsocketMessage> for WebSocketResponse<CoinbaseWebsocketMessage> {
    type Error = OpenLimitsError;

    fn try_from(value: CoinbaseWebsocketMessage) -> Result<Self> {
        match value {
            CoinbaseWebsocketMessage::Level2(level2) => {
                Ok(WebSocketResponse::Generic(level2.try_into()?))
            }
            _ => Ok(WebSocketResponse::Raw(value)),
        }
    }
}

impl TryFrom<Level2> for OpenLimitsWebSocketMessage {
    type Error = OpenLimitsError;

    fn try_from(level2: Level2) -> std::result::Result<Self, Self::Error> {
        // FIXME: How can we get the update id?
        let last_update_id = None;
        let update_id = None;
        Ok(match level2 {
            Level2::Snapshot { asks, bids, .. } => {
                let bids = bids.iter().map(|bid| bid.into()).collect();
                let asks = asks.iter().map(|ask| ask.into()).collect();
                let order_book_response = OrderBookResponse {
                    bids,
                    asks,
                    update_id,
                    last_update_id,
                };
                OpenLimitsWebSocketMessage::OrderBook(order_book_response)
            }
            Level2::L2update { changes, .. } => {
                let bids = changes
                    .iter()
                    .filter(|change| change.side == OrderSide::Buy)
                    .map(|change| change.into())
                    .collect();
                let asks = changes
                    .iter()
                    .filter(|change| change.side == OrderSide::Sell)
                    .map(|change| change.into())
                    .collect();
                let order_book_response = OrderBookResponse {
                    bids,
                    asks,
                    update_id,
                    last_update_id,
                };
                OpenLimitsWebSocketMessage::OrderBookDiff(order_book_response)
            }
        })
    }
}

impl From<&Level2SnapshotRecord> for AskBid {
    fn from(record: &Level2SnapshotRecord) -> Self {
        let price = record.price;
        let qty = record.size;
        Self { price, qty }
    }
}

impl From<&Level2UpdateRecord> for AskBid {
    fn from(record: &Level2UpdateRecord) -> Self {
        let price = record.price;
        let qty = record.size;
        Self { price, qty }
    }
}
