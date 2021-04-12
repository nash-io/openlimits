use super::{OrderBookData, TickerData, TradeData};
use crate::shared::string_to_decimal;
use rust_decimal::prelude::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExmoSubscription {
    // Public API
    Trades(String),             // symbol
    Ticker(String),             // symbol
    OrderBookSnapshots(String), // symbol
    OrderBookUpdates(String),   // symbol
    // Authenticated API
    UserTrades,
    Wallets,
    Orders,
}

#[derive(Debug, Clone, Serialize)]
pub enum ExmoWebsocketMessage {
    // General
    Greetings(GreetingsMessage),
    SubscribtionSuccess(SubscribtionSuccessMessage),
    SubscribtionError(SubscribtionErrorMessage),
    UnsubscribtionSuccess(UnsubscribtionSuccessMessage),
    Maintenance(MaintenanceMessage),
    LoginSuccess(AuthSuccessMessage),
    Ping,
    Pong,
    // Public
    Trades(TradeMessage),
    Ticker(TickerMessage),
    OrderBookSnapshots(OrderBookMessage),
    OrderBookUpdates(OrderBookMessage),
    // Authenticated
    UserTrades(UserTradeMessage),
    WalletsSnapshot(WalletSnapshotMessage),
    WalletsUpdate(WalletUpdateMessage),
    OrdersSnapshot(OrderSnapshotMessage),
    OrdersUpdate(OrderUpdateMessage),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GreetingsMessage {
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub event: String,
    pub code: i32,
    pub message: String,
    pub session_id: String,
}

// ???
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct SubscribeMessage {
//     pub id: u64,
//     pub method: String,
//     pub topics: Vec<String>,
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubscribtionSuccessMessage {
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub id: u64,
    pub event: String,
    pub topic: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubscribtionErrorMessage {
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub id: u64,
    pub event: String,
    pub code: i32,
    pub error: String,
}

// ???
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct UnsubscribeMessage {
//     pub id: u64,
//     pub method: String,
//     pub topics: Vec<String>,
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnsubscribtionSuccessMessage {
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub id: u64,
    pub event: String,
    pub topic: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MaintenanceMessage {
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub event: String,
    pub code: i32,
    pub message: String,
}

// ???
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Auth {
//     pub method: String,
//     pub id: u64,
//     pub api_key: String,
//     pub sign: String,
//     pub nonce: String,
// }

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthSuccessMessage {
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub id: u64,
    pub event: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeMessage {
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub event: String,
    pub topic: String,
    pub data: Vec<TradeData>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TickerMessage {
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub event: String,
    pub topic: String,
    pub data: TickerData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderBookMessage {
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub event: String,
    pub topic: String,
    pub data: OrderBookData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserTradeData {
    pub trade_id: u64,
    #[serde(rename = "type")]
    pub event_type: String,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub quantity: Decimal,
    #[serde(with = "string_to_decimal")]
    pub amount: Decimal,
    pub date: u64,
    pub order_id: u64,
    pub pair: String,
    pub exec_type: String,
    #[serde(with = "string_to_decimal")]
    pub commission_amount: Decimal,
    pub commission_currency: String,
    #[serde(with = "string_to_decimal")]
    pub commission_percent: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserTradeMessage {
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub event: String,
    pub topic: String,
    pub data: UserTradeData,
}

// ???
// TODO: proper deserialization
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletSnapshotData {
    pub balances: String,
    pub reserved: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletSnapshotMessage {
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub event: String,
    pub topic: String,
    pub data: WalletSnapshotData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletUpdateData {
    pub currency: String,
    #[serde(with = "string_to_decimal")]
    pub ballance: Decimal,
    #[serde(with = "string_to_decimal")]
    pub reserved: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletUpdateMessage {
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub event: String,
    pub topic: String,
    pub data: WalletUpdateData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderSnapshotData {
    pub order_id: String,
    pub client_id: String,
    pub pair: String,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub quantity: Decimal,
    #[serde(with = "string_to_decimal")]
    pub amount: Decimal,
    #[serde(with = "string_to_decimal")]
    pub original_quantity: Decimal,
    #[serde(with = "string_to_decimal")]
    pub original_amount: Decimal,
    #[serde(rename = "type")]
    pub event_type: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParentOrderSnapshotData {
    pub parent_order_id: String,
    pub client_id: String,
    pub created: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub pair: String,
    #[serde(with = "string_to_decimal")]
    pub quantity: Decimal,
    #[serde(with = "string_to_decimal")]
    pub trigger_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub amount: Decimal,
    pub status: String,
    #[serde(with = "string_to_decimal")]
    pub reserved: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderSnapshotMessage {
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub event: String,
    pub topic: String,
    pub data: Vec<(OrderSnapshotData, ParentOrderSnapshotData)>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderUpdateData {
    pub order_id: String,
    pub client_id: String,
    pub pair: String,
    #[serde(with = "string_to_decimal")]
    pub quantity: Decimal,
    #[serde(with = "string_to_decimal")]
    pub original_quantity: Decimal,
    #[serde(rename = "type")]
    pub event_type: String,
    pub status: String,
    pub last_trade_id: String,
    #[serde(with = "string_to_decimal")]
    pub last_trade_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub last_trade_quantity: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderUpdateMessage {
    #[serde(rename = "ts")]
    pub timestamp: u64,
    pub event: String,
    pub topic: String,
    pub data: Vec<OrderUpdateData>,
}
