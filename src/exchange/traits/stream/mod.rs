//! Used to instatiate websockets 
//! # Example
//! ```
//! use openlimits::traits::stream::OpenLimitWs;
//! use openlimits::exchange::binance::BinanceWebsocket;
//! use openlimits::exchange::binance::BinanceParameters;
//! 
//! let mut binance_websocket = OpenLimitsWs {
//!     websocket: BinanceWebsocket::new(BinanceParameters::prod())
//!         .await
//!         .expect("Failed to create Client"),
//! };
//! ```

use std::convert::TryFrom;
use crate::errors::OpenLimitsError;
use crate::model::websocket::WebSocketResponse;
use crate::model::websocket::OpenLimitsWebSocketMessage;
use super::shared::Result;

mod callback_handle;
mod exchange_ws;
mod open_limit_ws;
mod subscriptions;

pub use callback_handle::CallbackHandle;
pub use exchange_ws::ExchangeWs;
pub use open_limit_ws::OpenLimitsWs;
pub use subscriptions::Subscriptions;
pub use super::shared;

impl TryFrom<OpenLimitsWebSocketMessage> for WebSocketResponse<OpenLimitsWebSocketMessage> {
    type Error = OpenLimitsError;

    fn try_from(value: OpenLimitsWebSocketMessage) -> Result<Self> {
        Ok(WebSocketResponse::Generic(value))
    }
}

