pub mod websocket;
use crate::shared::string_to_decimal;
use rust_decimal::prelude::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TradeData {
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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderBookPosition(#[serde(with = "string_to_decimal")] Decimal);

// Order book position format is JSON array of 3 numbers:
// ["price","quantity","amount"]
// https://documenter.getpostman.com/view/10287440/SzYXWKPi#308cf5d9-773b-48c0-8e0d-5a1c2ca40751
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderBookData {
    pub ask: Vec<[OrderBookPosition; 3]>,
    pub bid: Vec<[OrderBookPosition; 3]>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TickerData {
    #[serde(with = "string_to_decimal")]
    pub buy_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub sell_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub last_trade: Decimal,
    #[serde(with = "string_to_decimal")]
    pub high: Decimal,
    #[serde(with = "string_to_decimal")]
    pub low: Decimal,
    #[serde(with = "string_to_decimal")]
    pub avg: Decimal,
    #[serde(with = "string_to_decimal")]
    pub vol: Decimal,
    #[serde(with = "string_to_decimal")]
    pub vol_curr: Decimal,
    pub updated: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    #[serde(with = "string_to_decimal")]
    pub min_quantity: Decimal,
    #[serde(with = "string_to_decimal")]
    pub max_quantity: Decimal,
    #[serde(with = "string_to_decimal")]
    pub min_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub max_price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub min_amount: Decimal,
    #[serde(with = "string_to_decimal")]
    pub max_amount: Decimal,
    pub price_precision: i32,
    #[serde(with = "string_to_decimal")]
    pub commission_taker_percent: Decimal,
    #[serde(with = "string_to_decimal")]
    pub commission_maker_percent: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PairSettings(HashMap<String, Settings>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Currency(Vec<String>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrencyData {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrencyListExtended(Vec<CurrencyData>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequiredAmount {
    #[serde(with = "string_to_decimal")]
    pub quantity: Decimal,
    #[serde(with = "string_to_decimal")]
    pub amount: Decimal,
    #[serde(with = "string_to_decimal")]
    pub avg_price: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Candle {
    #[serde(rename = "t")]
    pub timestamp: u64,
    #[serde(rename = "o", with = "string_to_decimal")]
    pub open: Decimal,
    #[serde(rename = "c", with = "string_to_decimal")]
    pub close: Decimal,
    #[serde(rename = "h", with = "string_to_decimal")]
    pub high: Decimal,
    #[serde(rename = "l", with = "string_to_decimal")]
    pub low: Decimal,
    #[serde(rename = "v", with = "string_to_decimal")]
    pub volume: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CandlesHistory {
    pub candles: Vec<Candle>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Provider {
    #[serde(rename = "type")]
    pub provider_method: String,
    pub name: String,
    pub currency_name: String,
    #[serde(with = "string_to_decimal")]
    pub min: Decimal,
    #[serde(with = "string_to_decimal")]
    pub max: Decimal,
    pub enabled: bool,
    pub comment: String,
    pub commission_desc: String,
    pub currency_confirmations: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaymentsProvidersCryptoList(HashMap<String, Settings>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CurrencyBallance(#[serde(with = "string_to_decimal")] Decimal);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfo {
    pub uid: u64,
    pub server_date: u64,
    pub balances: HashMap<String, CurrencyBallance>,
    pub reserved: HashMap<String, CurrencyBallance>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderCreate {
    pub result: bool,
    pub error: String,
    pub order_id: u64,
    pub client_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderCancel {
    pub result: bool,
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StopMarketOrderCreate {
    pub client_id: u64,
    pub parent_order_id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParentOrderData {
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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserOrderData {
    pub order_id: String,
    pub client_id: String,
    pub created: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub pair: String,
    #[serde(with = "string_to_decimal")]
    pub quantity: Decimal,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub amount: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserOpenOrders(
    HashMap<String, Vec<ParentOrderData>>,
    HashMap<String, Vec<UserOrderData>>,
);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserBuyDeals {
    pub trade_id: u64,
    pub client_id: u64,
    pub date: u64,
    #[serde(rename = "type")]
    pub event_type: String,
    pub pair: String,
    #[serde(with = "string_to_decimal")]
    pub quantity: Decimal,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub amount: Decimal,
    pub order_id: u64,
    pub parent_order_id: u64,
    pub exec_type: String,
    #[serde(with = "string_to_decimal")]
    pub commission_amount: Decimal,
    pub commission_currency: String,
    #[serde(with = "string_to_decimal")]
    pub commission_percent: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSellDeals {
    pub trade_id: u64,
    pub client_id: u64,
    pub date: u64,
    #[serde(rename = "type")]
    pub event_type: String,
    pub pair: String,
    #[serde(with = "string_to_decimal")]
    pub quantity: Decimal,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub amount: Decimal,
    pub order_id: u64,
    pub exec_type: String,
    #[serde(with = "string_to_decimal")]
    pub commission_amount: Decimal,
    pub commission_currency: String,
    #[serde(with = "string_to_decimal")]
    pub commission_percent: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserDeals(
    HashMap<String, Vec<UserSellDeals>>,
    HashMap<String, Vec<UserBuyDeals>>,
);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserCancelParent {
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
    pub reason_status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserCancelOrder {
    pub order_id: String,
    pub client_id: String,
    pub created: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub pair: String,
    #[serde(with = "string_to_decimal")]
    pub quantity: Decimal,
    #[serde(with = "string_to_decimal")]
    pub price: Decimal,
    #[serde(with = "string_to_decimal")]
    pub amount: Decimal,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserCanceledOrders(Vec<(UserCancelParent, UserCancelOrder)>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderTrades {
    #[serde(rename = "type")]
    pub event_type: String,
    pub in_currency: String,
    #[serde(with = "string_to_decimal")]
    pub in_amount: Decimal,
    pub out_currency: String,
    #[serde(with = "string_to_decimal")]
    pub out_amount: Decimal,
    pub trades: Vec<UserSellDeals>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepositAddress(HashMap<String, String>);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WithdrawCrypt {
    pub result: bool,
    pub error: String,
    pub task_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WithdrawGetTxId {
    pub result: bool,
    pub error: String,
    pub status: bool,
    pub txid: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExcodeCreate {
    pub result: bool,
    pub error: String,
    pub task_id: String,
    pub code: String,
    #[serde(with = "string_to_decimal")]
    pub amount: Decimal,
    pub currency: String,
    pub login: String,
    #[serde(with = "string_to_decimal")]
    pub commission: Decimal,
    pub balances: HashMap<String, Decimal>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExcodeLoad {
    pub result: bool,
    pub error: String,
    pub task_id: String,
    #[serde(with = "string_to_decimal")]
    pub amount: Decimal,
    pub currency: String,
    pub reviewing: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodeCheck {
    pub result: bool,
    pub error: String,
    pub valid: bool,
    pub created: u64,
    pub used: bool,
    pub used_dt: u64,
    #[serde(with = "string_to_decimal")]
    pub amount: Decimal,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletHistoryRecord {
    pub dt: u64,
    #[serde(rename = "type")]
    pub event_type: String,
    pub curr: String,
    pub status: String,
    pub provider: String,
    #[serde(with = "string_to_decimal")]
    pub amount: Decimal,
    pub txid: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletHistory {
    pub result: bool,
    pub error: String,
    pub begin: String,
    pub end: String,
    pub history: Vec<WalletHistoryRecord>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OperationExtra {
    pub txid: String,
    pub excode: String,
    pub invoice: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Operation {
    pub operation_id: u64,
    pub created: u64,
    pub updated: u64,
    #[serde(rename = "type")]
    pub event_type: String,
    pub currency: String,
    pub status: String,
    #[serde(with = "string_to_decimal")]
    pub amount: Decimal,
    pub provider: String,
    #[serde(with = "string_to_decimal")]
    pub commission: Decimal,
    pub account: String,
    pub order_id: u64,
    pub extra: OperationExtra,
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WalletOperations {
    pub items: Vec<Operation>,
    pub count: i32,
}
