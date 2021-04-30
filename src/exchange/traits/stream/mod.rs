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

