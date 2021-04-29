pub const ORDER_TYPE_LIMIT: &str = "LIMIT";
pub const ORDER_TYPE_LIMIT_MAKER: &str = "LIMIT_MAKER";
pub const ORDER_TYPE_MARKET: &str = "MARKET";
pub const ORDER_SIDE_BUY: &str = "BUY";
pub const ORDER_SIDE_SELL: &str = "SELL";
pub const TIME_IN_FORCE_GTC: &str = "GTC";

mod account_information;
mod all_order_req;
mod ask_bid;
mod balance;
mod book_tickers;
mod exchange_filter;
mod exchange_information;
mod interval;
mod kline;
mod kline_params;
mod kline_summaries;
mod kline_summary;
mod order;
mod order_book;
mod order_canceled;
mod order_exec_type;
mod order_reject_reason;
mod order_request;
mod order_status;
mod order_type;
mod paginator;
mod price_stats;
mod prices;
mod rate_limit;
mod rate_limit_type;
mod server_time;
mod side;
mod success;
mod symbol;
mod symbol_filter;
mod symbol_price;
mod ticker;
mod time_in_force;
mod trade_history;
mod trade_history_req;
mod transaction;
mod user_data_stream;
pub mod websocket;

pub use account_information::AccountInformation;
pub use all_order_req::AllOrderReq;
pub use ask_bid::AskBid;
pub use balance::Balance;
pub use book_tickers::BookTickers;
pub use exchange_filter::ExchangeFilter;
pub use exchange_information::ExchangeInformation;
pub use interval::Interval;
pub use kline::Kline;
pub use kline_params::KlineParams;
pub use kline_summaries::KlineSummaries;
pub use kline_summary::KlineSummary;
pub use order::Order;
pub use order_book::OrderBook;
pub use order_canceled::OrderCanceled;
pub use order_exec_type::OrderExecType;
pub use order_reject_reason::OrderRejectReason;
pub use order_request::OrderRequest;
pub use order_status::OrderStatus;
pub use order_type::OrderType;
pub use paginator::Paginator;
pub use price_stats::PriceStats;
pub use prices::Prices;
pub use rate_limit::RateLimit;
pub use rate_limit_type::RateLimitType;
pub use server_time::ServerTime;
pub use side::Side;
pub use success::Success;
pub use symbol::Symbol;
pub use symbol_filter::SymbolFilter;
pub use symbol_price::SymbolPrice;
pub use ticker::Ticker;
pub use time_in_force::TimeInForce;
pub use trade_history::TradeHistory;
pub use trade_history_req::TradeHistoryReq;
pub use transaction::Transaction;
pub use user_data_stream::UserDataStream;

/*
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    pub server_time: u64,
}
*/

/*
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeInformation {
    pub timezone: String,
    pub server_time: u64,
    pub rate_limits: Vec<RateLimit>,
    pub exchange_filters: Vec<ExchangeFilter>,
    pub symbols: Vec<Symbol>,
}
*/

/*
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
*/

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Balance {
//     pub asset: String,
//     #[serde(with = "string_to_decimal")]
//     pub free: Decimal,
//     #[serde(with = "string_to_decimal")]
//     pub locked: Decimal,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Order {
//     pub symbol: String,
//     pub order_id: u64,
//     pub client_order_id: String,
//     #[serde(with = "string_to_decimal")]
//     pub price: Decimal,
//     #[serde(with = "string_to_decimal")]
//     pub orig_qty: Decimal,
//     #[serde(with = "string_to_decimal")]
//     pub executed_qty: Decimal,
//     pub status: OrderStatus,
//     pub time_in_force: String,
//     #[serde(rename = "type")]
//     pub type_name: String,
//     pub side: String,
//     #[serde(with = "string_to_opt_decimal")]
//     #[serde(default)]
//     pub stop_price: Option<Decimal>,
//     #[serde(default)]
//     pub iceberg_qty: Option<String>,
//     #[serde(default)]
//     pub time: Option<u64>,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct OrderRequest {
//     pub symbol: String,
//     #[serde(with = "string_to_decimal")]
//     pub quantity: Decimal,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     #[serde(with = "string_to_opt_decimal")]
//     pub price: Option<Decimal>,
//     #[serde(rename = "side")]
//     pub order_side: String,
//     #[serde(rename = "type")]
//     pub order_type: String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub time_in_force: Option<TimeInForce>,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct OrderCanceled {
//     pub symbol: String,
//     pub orig_client_order_id: String,
//     pub order_id: u64,
//     pub client_order_id: String,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Transaction {
//     pub symbol: String,
//     pub order_id: u64,
//     pub client_order_id: String,
//     pub transact_time: u64,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct AskBid {
//     #[serde(with = "string_to_decimal")]
//     pub price: Decimal,
//     #[serde(with = "string_to_decimal")]
//     pub qty: Decimal,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct UserDataStream {
//     pub listen_key: String,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct Success {}

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// #[serde(untagged)]
// pub enum Prices {
//     AllPrices(Vec<SymbolPrice>),
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct SymbolPrice {
//     pub symbol: String,
//     #[serde(with = "string_to_decimal")]
//     pub price: Decimal,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// #[serde(untagged)]
// pub enum BookTickers {
//     AllBookTickers(Vec<Ticker>),
// }

// #[derive(Debug, Clone)]
// pub enum KlineSummaries {
//     AllKlineSummaries(Vec<KlineSummary>),
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Ticker {
//     pub symbol: String,
//     #[serde(with = "string_to_decimal")]
//     pub bid_price: Decimal,
//     #[serde(with = "string_to_decimal")]
//     pub bid_qty: Decimal,
//     #[serde(with = "string_to_decimal")]
//     pub ask_price: Decimal,
//     #[serde(with = "string_to_decimal")]
//     pub ask_qty: Decimal,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct KlineParams {
//     pub symbol: String,
//     pub interval: String,
//     #[serde(flatten)]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub paginator: Option<Paginator>,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct TradeHistory {
//     pub symbol: String,
//     pub id: u64,
//     pub order_id: u64,
//     #[serde(with = "string_to_decimal")]
//     pub price: Decimal,
//     #[serde(with = "string_to_decimal")]
//     pub qty: Decimal,
//     #[serde(with = "string_to_decimal")]
//     pub commission: Decimal,
//     pub commission_asset: String,
//     pub time: u64,
//     pub is_buyer: bool,
//     pub is_maker: bool,
//     pub is_best_match: bool,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct PriceStats {
//     pub symbol: String,
//     #[serde(with = "string_to_decimal")]
//     pub price_change: Decimal,
//     #[serde(with = "string_to_decimal")]
//     pub price_change_percent: Decimal,
//     #[serde(with = "string_to_decimal")]
//     pub weighted_avg_price: Decimal,
//     #[serde(with = "string_to_decimal")]
//     pub prev_close_price: Decimal,
//     #[serde(with = "string_to_decimal")]
//     pub last_price: Decimal,
//     #[serde(with = "string_to_decimal")]
//     pub bid_price: Decimal,
//     #[serde(with = "string_to_decimal")]
//     pub ask_price: Decimal,
//     #[serde(with = "string_to_decimal")]
//     pub open_price: Decimal,
//     #[serde(with = "string_to_decimal")]
//     pub high_price: Decimal,
//     #[serde(with = "string_to_decimal")]
//     pub low_price: Decimal,
//     #[serde(with = "string_to_decimal")]
//     pub volume: Decimal,
//     pub open_time: u64,
//     pub close_time: u64,
//     pub first_id: i64, // For dummy symbol "123456", it is -1
//     pub last_id: i64,  // Same as above
//     pub count: u64,
// }

// #[derive(Debug, Clone)]
// pub struct KlineSummary {
//     pub open_time: i64,

//     pub open: Decimal,

//     pub high: Decimal,

//     pub low: Decimal,

//     pub close: Decimal,

//     pub volume: Decimal,

//     pub close_time: i64,

//     pub quote_asset_volume: Decimal,

//     pub number_of_trades: i64,

//     pub taker_buy_base_asset_volume: Decimal,

//     pub taker_buy_quote_asset_volume: Decimal,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Kline {
//     #[serde(rename = "t")]
//     pub start_time: i64,
//     #[serde(rename = "T")]
//     pub end_time: i64,
//     #[serde(rename = "s")]
//     pub symbol: String,
//     #[serde(rename = "i")]
//     pub interval: String,
//     #[serde(rename = "f")]
//     pub first_trade_id: i32,
//     #[serde(rename = "L")]
//     pub last_trade_id: i32,
//     #[serde(rename = "o")]
//     pub open: String,
//     #[serde(rename = "c")]
//     pub close: String,
//     #[serde(rename = "h")]
//     pub high: String,
//     #[serde(rename = "l")]
//     pub low: String,
//     #[serde(rename = "v")]
//     pub volume: String,
//     #[serde(rename = "n")]
//     pub number_of_trades: i32,
//     #[serde(rename = "x")]
//     pub is_final_bar: bool,
//     #[serde(rename = "q")]
//     pub quote_volume: String,
//     #[serde(rename = "V")]
//     pub active_buy_volume: String,
//     #[serde(rename = "Q")]
//     pub active_volume_buy_quote: String,
//     #[serde(skip_serializing, rename = "B")]
//     pub ignore_me: String,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct RateLimit {
//     rate_limit_type: RateLimitType,
//     interval: Interval,
//     limit: u64,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum RateLimitType {
//     Orders,
//     RequestWeight,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum Interval {
//     Second,
//     Minute,
//     Day,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(tag = "filterType", rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum SymbolFilter {
//     #[serde(rename_all = "camelCase")]
//     LotSize {
//         #[serde(with = "string_to_decimal")]
//         min_qty: Decimal,
//         #[serde(with = "string_to_decimal")]
//         max_qty: Decimal,
//         #[serde(with = "string_to_decimal")]
//         step_size: Decimal,
//     },
//     #[serde(rename_all = "camelCase")]
//     PriceFilter {
//         #[serde(with = "string_to_decimal")]
//         min_price: Decimal,
//         #[serde(with = "string_to_decimal")]
//         max_price: Decimal,
//         #[serde(with = "string_to_decimal")]
//         tick_size: Decimal,
//     },
//     #[serde(rename_all = "camelCase")]
//     MinNotional {
//         #[serde(with = "string_to_decimal")]
//         min_notional: Decimal,
//     },
//     #[serde(rename_all = "camelCase")]
//     MaxNumAlgoOrders { max_num_algo_orders: u64 },
//     #[serde(rename_all = "camelCase")]
//     MaxPosition {
//         #[serde(with = "string_to_decimal")]
//         max_position: Decimal,
//     },
//     #[serde(rename_all = "camelCase")]
//     MaxNumOrders { max_num_orders: u64 },
//     #[serde(rename_all = "camelCase")]
//     IcebergParts { limit: u64 },
//     #[serde(rename_all = "camelCase")]
//     PercentPrice {
//         #[serde(with = "string_to_decimal")]
//         multiplier_up: Decimal,
//         #[serde(with = "string_to_decimal")]
//         multiplier_down: Decimal,
//         avg_price_mins: u64,
//     },
//     #[serde(rename_all = "camelCase")]
//     MarketLotSize {
//         #[serde(with = "string_to_decimal")]
//         min_qty: Decimal,
//         #[serde(with = "string_to_decimal")]
//         max_qty: Decimal,
//         #[serde(with = "string_to_decimal")]
//         step_size: Decimal,
//     },
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(tag = "filterType", rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum ExchangeFilter {
//     #[serde(rename_all = "camelCase")]
//     ExchangeMaxNumOrders { max_num_orders: u64 },
//     #[serde(rename_all = "camelCase")]
//     ExchangeMaxNumAlgoOrders { max_num_algo_orders: u64 },
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct Symbol {
//     pub symbol: String,
//     pub status: String,
//     pub base_asset: String,
//     pub base_asset_precision: u32,
//     pub quote_asset: String,
//     pub quote_precision: u32,
//     pub order_types: Vec<String>,
//     pub iceberg_allowed: bool,
//     pub filters: Vec<SymbolFilter>,
// }

// #[derive(Debug, Serialize, Deserialize, Clone)]
// #[serde(rename_all = "camelCase")]
// pub struct OrderBook {
//     pub last_update_id: u64,
//     pub bids: Vec<AskBid>,
//     pub asks: Vec<AskBid>,
// }

// #[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum Side {
//     Buy,
//     Sell,
// }

// #[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum OrderType {
//     Market,
//     Limit,
//     StopLoss,
//     StopLossLimit,
//     TakeProfit,
//     TakeProfitLimit,
//     LimitMaker,
// }

// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub enum TimeInForce {
//     GTC,
//     IOC,
//     FOK,
// }

// #[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum OrderExecType {
//     New,
// }

// #[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum OrderStatus {
//     New,
//     PartiallyFilled,
//     Filled,
//     Canceled,
//     PendingCancel,
//     Rejected,
//     Expired,
// }

// #[derive(Serialize, Deserialize, Clone, Debug)]
// #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
// pub enum OrderRejectReason {
//     None,
// }

// #[derive(Clone, Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct AllOrderReq {
//     pub symbol: String,
//     #[serde(flatten)]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub paginator: Option<Paginator>,
// }

// #[derive(Clone, Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct TradeHistoryReq {
//     pub symbol: String,
//     #[serde(flatten)]
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub paginator: Option<Paginator>,
// }

// #[derive(Clone, Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct Paginator {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub start_time: Option<u64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub end_time: Option<u64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub limit: Option<u64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub from_id: Option<u64>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub order_id: Option<u64>,
// }
