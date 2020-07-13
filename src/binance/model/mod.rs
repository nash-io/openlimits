pub mod websocket;
use crate::utils::string_or_float_to_float;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub server_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformation {
    pub timezone: String,
    pub server_time: u64,
    pub rate_limits: Vec<RateLimit>,
    pub symbols: Vec<Symbol>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInformation {
    pub maker_commission: f32,
    pub taker_commission: f32,
    pub buyer_commission: f32,
    pub seller_commission: f32,
    pub can_trade: bool,
    pub can_withdraw: bool,
    pub can_deposit: bool,
    pub balances: Vec<Balance>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    pub asset: String,
    pub free: String,
    pub locked: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub symbol: String,
    pub order_id: u64,
    pub client_order_id: String,
    #[serde(with = "string_or_float_to_float")]
    pub price: f64,
    pub orig_qty: String,
    pub executed_qty: String,
    pub status: String,
    pub time_in_force: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub side: String,
    #[serde(with = "string_or_float_to_float")]
    pub stop_price: f64,
    pub iceberg_qty: String,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderCanceled {
    pub symbol: String,
    pub orig_client_order_id: String,
    pub order_id: u64,
    pub client_order_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub symbol: String,
    pub order_id: u64,
    pub client_order_id: String,
    pub transact_time: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bids {
    #[serde(with = "string_or_float_to_float")]
    pub price: f64,
    #[serde(with = "string_or_float_to_float")]
    pub qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asks {
    #[serde(with = "string_or_float_to_float")]
    pub price: f64,
    #[serde(with = "string_or_float_to_float")]
    pub qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDataStream {
    pub listen_key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Success {}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum Prices {
    AllPrices(Vec<SymbolPrice>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymbolPrice {
    pub symbol: String,
    #[serde(with = "string_or_float_to_float")]
    pub price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum BookTickers {
    AllBookTickers(Vec<Ticker>),
}

#[derive(Debug, Clone)]
pub enum KlineSummaries {
    AllKlineSummaries(Vec<KlineSummary>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    pub symbol: String,
    #[serde(with = "string_or_float_to_float")]
    pub bid_price: f64,
    #[serde(with = "string_or_float_to_float")]
    pub bid_qty: f64,
    #[serde(with = "string_or_float_to_float")]
    pub ask_price: f64,
    #[serde(with = "string_or_float_to_float")]
    pub ask_qty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TradeHistory {
    pub symbol: String,
    pub id: u64,
    pub order_id: u64,
    #[serde(with = "string_or_float_to_float")]
    pub price: f64,
    #[serde(with = "string_or_float_to_float")]
    pub qty: f64,
    #[serde(with = "string_or_float_to_float")]
    pub commission: f64,
    pub commission_asset: String,
    pub time: u64,
    pub is_buyer: bool,
    pub is_maker: bool,
    pub is_best_match: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PriceStats {
    pub symbol: String,
    #[serde(with = "string_or_float_to_float")]
    pub price_change: f64,
    #[serde(with = "string_or_float_to_float")]
    pub price_change_percent: f64,
    #[serde(with = "string_or_float_to_float")]
    pub weighted_avg_price: f64,
    #[serde(with = "string_or_float_to_float")]
    pub prev_close_price: f64,
    #[serde(with = "string_or_float_to_float")]
    pub last_price: f64,
    #[serde(with = "string_or_float_to_float")]
    pub bid_price: f64,
    #[serde(with = "string_or_float_to_float")]
    pub ask_price: f64,
    #[serde(with = "string_or_float_to_float")]
    pub open_price: f64,
    #[serde(with = "string_or_float_to_float")]
    pub high_price: f64,
    #[serde(with = "string_or_float_to_float")]
    pub low_price: f64,
    #[serde(with = "string_or_float_to_float")]
    pub volume: f64,
    pub open_time: u64,
    pub close_time: u64,
    pub first_id: i64, // For dummy symbol "123456", it is -1
    pub last_id: i64,  // Same as above
    pub count: u64,
}

#[derive(Debug, Clone)]
pub struct KlineSummary {
    pub open_time: i64,

    pub open: f64,

    pub high: f64,

    pub low: f64,

    pub close: f64,

    pub volume: f64,

    pub close_time: i64,

    pub quote_asset_volume: f64,

    pub number_of_trades: i64,

    pub taker_buy_base_asset_volume: f64,

    pub taker_buy_quote_asset_volume: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    #[serde(rename = "t")]
    pub start_time: i64,
    #[serde(rename = "T")]
    pub end_time: i64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: String,
    #[serde(rename = "f")]
    pub first_trade_id: i32,
    #[serde(rename = "L")]
    pub last_trade_id: i32,
    #[serde(rename = "o")]
    pub open: String,
    #[serde(rename = "c")]
    pub close: String,
    #[serde(rename = "h")]
    pub high: String,
    #[serde(rename = "l")]
    pub low: String,
    #[serde(rename = "v")]
    pub volume: String,
    #[serde(rename = "n")]
    pub number_of_trades: i32,
    #[serde(rename = "x")]
    pub is_final_bar: bool,
    #[serde(rename = "q")]
    pub quote_volume: String,
    #[serde(rename = "V")]
    pub active_buy_volume: String,
    #[serde(rename = "Q")]
    pub active_volume_buy_quote: String,
    #[serde(skip_serializing, rename = "B")]
    pub ignore_me: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RateLimit {
    rate_limit_type: RateLimitType,
    interval: Interval,
    limit: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RateLimitType {
    Orders,
    RequestWeight,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Interval {
    Second,
    Minute,
    Day,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "filterType", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SymbolFilter {
    #[serde(rename_all = "camelCase")]
    LotSize {
        min_qty: String,
        max_qty: String,
        step_size: String,
    },
    #[serde(rename_all = "camelCase")]
    PriceFilter {
        min_price: String,
        max_price: String,
        tick_size: String,
    },
    #[serde(rename_all = "camelCase")]
    MinNotional { min_notional: String },
    #[serde(rename_all = "camelCase")]
    MaxNumAlgoOrders { max_num_algo_orders: u64 },
    #[serde(rename_all = "camelCase")]
    MaxNumOrders { max_num_orders: u64 },
    #[serde(rename_all = "camelCase")]
    IcebergParts { limit: u64 },
    #[serde(rename_all = "camelCase")]
    PercentPrice {
        multiplier_up: String,
        multiplier_down: String,
        avg_price_mins: u64,
    },
    #[serde(rename_all = "camelCase")]
    MarketLotSize {
        min_qty: String,
        max_qty: String,
        step_size: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "filterType", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExchangeFilter {
    ExchangeMaxNumOrders { limit: u64 },
    ExchangeMaxAlgoOrders { limit: u64 },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub status: String,
    pub base_asset: String,
    pub base_asset_precision: u64,
    pub quote_asset: String,
    pub quote_precision: u64,
    pub order_types: Vec<String>,
    pub iceberg_allowed: bool,
    pub filters: Vec<SymbolFilter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderBook {
    pub last_update_id: u64,
    pub bids: Vec<Bids>,
    pub asks: Vec<Asks>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
    LimitMaker,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeInForce {
    GTC,
    IOC,
    FOK,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderExecType {
    New,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    PendingCancel,
    Rejected,
    Expired,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderRejectReason {
    None,
}
