
#[no_mangle]
pub  extern "cdecl" fn say_something() {
    println!("Saying something");
}

// use std::{convert::{TryInto}};
// use rust_decimal::prelude::*;
// use rust_decimal::Decimal;
// use chrono::Duration;
// use openlimits::{
//   OpenLimits,
//   exchange::{
//     traits::{
//       ExchangeAccount,
//       ExchangeMarketData,
//       stream::OpenLimitsWs,
//       info::{MarketPair, ExchangeInfoRetrieval}
//     },
//     // nash::{
//     //   NashCredentials,
//     //   NashParameters,
//     //   Environment
//     // },
//     binance::{
//       BinanceCredentials,
//       BinanceParameters,
//     },
//     coinbase::{
//       CoinbaseCredentials,
//       CoinbaseParameters,
//     },
//     model::{
//       OrderBookRequest,
//       GetOrderRequest,
//       Liquidity,
//       Side,
//       CancelAllOrdersRequest,
//       CancelOrderRequest,
//       OrderType,
//       AskBid,
//       TimeInForce,
//       OpenLimitOrderRequest,
//       OrderStatus,
//       OpenMarketOrderRequest,
//       GetOrderHistoryRequest,
//       TradeHistoryRequest,
//       GetHistoricTradesRequest,
//       GetHistoricRatesRequest,
//       GetPriceTickerRequest,
//       Paginator,
//       Balance,
//       Order,
//       Trade,
//       Interval,
//       Candle,
//       websocket::{Subscription, OpenLimitsWebSocketMessage, WebSocketResponse}
//     }
//   },
//   errors::OpenLimitsError,
//   // any_exchange::{AnyExchange, InitAnyExchange, AnyWsExchange},
// };
// use tokio::stream::StreamExt;
// use std::{ffi::CStr, ffi::CString, os::raw::c_char};
// use thiserror::Error;
//
// #[repr(u32)]
// #[derive(Debug, Copy, Clone)]
// pub enum FFIInterval {
//   OneMinute,
//   ThreeMinutes,
//   FiveMinutes,
//   FifteenMinutes,
//   ThirtyMinutes,
//   OneHour,
//   TwoHours,
//   FourHours,
//   SixHours,
//   EightHours,
//   TwelveHours,
//   OneDay,
//   ThreeDays,
//   OneWeek,
//   OneMonth
// }
//
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct FFIMarketPair {
//   base: *mut c_char,
//   quote: *mut c_char,
//   symbol: *mut c_char,
//   base_increment: *mut c_char,
//   quote_increment: *mut c_char,
//   base_min_price: *mut c_char,
//   quote_min_price: *mut c_char,
// }
//
// fn interval_from_ffi_interval(
//   interval: FFIInterval
// ) -> Result<Interval, String> {
//   #[allow(unreachable_patterns)]
//   match interval {
//     FFIInterval::OneMinute => Ok(Interval::OneMinute),
//     FFIInterval::ThreeMinutes => Ok(Interval::ThreeMinutes),
//     FFIInterval::FiveMinutes => Ok(Interval::FiveMinutes),
//     FFIInterval::FifteenMinutes => Ok(Interval::FifteenMinutes),
//     FFIInterval::ThirtyMinutes => Ok(Interval::ThirtyMinutes),
//     FFIInterval::OneHour => Ok(Interval::OneHour),
//     FFIInterval::TwoHours => Ok(Interval::TwoHours),
//     FFIInterval::FourHours => Ok(Interval::FourHours),
//     FFIInterval::SixHours => Ok(Interval::SixHours),
//     FFIInterval::EightHours => Ok(Interval::EightHours),
//     FFIInterval::TwelveHours => Ok(Interval::TwelveHours),
//     FFIInterval::OneDay => Ok(Interval::OneDay),
//     FFIInterval::ThreeDays => Ok(Interval::ThreeDays),
//     FFIInterval::OneWeek => Ok(Interval::OneWeek),
//     FFIInterval::OneMonth => Ok(Interval::OneMonth),
//     _ => Err(format!("Invalid interval value {:?}", interval))
//   }
// }
//
//
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct FFICandle {
//   time: u64,
//   low: f64,
//   high: f64,
//   open: f64,
//   close: f64,
//   volume: f64,
// }
//
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct FFIPaginator {
//   start_time: u64,
//   end_time: u64,
//   limit: u64,
//   before: *mut c_char,
//   after: *mut c_char,
// }
//
//
// fn string_to_c_str(s: String) -> *mut c_char {
//   let cex = CString::new(s).expect("Failed to create CString!");
//   let raw = cex.into_raw();
//   // println!("Handling ownership of {:?} to c#", raw);
//
//   raw
// }
//
//
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct FFIBalance {
//   asset: *mut c_char,
//   total: *mut c_char,
//   free: *mut c_char,
// }
//
// fn to_ffi_balance(b: Balance) -> FFIBalance {
//   FFIBalance {
//     asset: string_to_c_str(b.asset),
//     total: string_to_c_str(b.total.to_string()),
//     free: string_to_c_str(b.free.to_string())
//   }
// }
//
// fn market_pair_to_ffi(pair: MarketPair) -> FFIMarketPair {
//   let base_min_price = pair.min_base_trade_size.map(|f|string_to_c_str(f.to_string())).unwrap_or(std::ptr::null_mut());
//   let quote_min_price = pair.min_quote_trade_size.map(|f|string_to_c_str(f.to_string())).unwrap_or(std::ptr::null_mut());
//
//   FFIMarketPair {
//     base: string_to_c_str(pair.base),
//     quote: string_to_c_str(pair.quote),
//     symbol: string_to_c_str(pair.symbol),
//     base_increment: string_to_c_str(pair.base_increment.to_string()),
//     quote_increment: string_to_c_str(pair.quote_increment.to_string()),
//     base_min_price,
//     quote_min_price,
//   }
// }
//
// fn c_str_to_string(s: *mut c_char) -> Result<String, std::str::Utf8Error> {
//   let str = unsafe { CStr::from_ptr(s) };
//   str.to_str().map(String::from)
// }
// fn nullable_cstr(s: *mut c_char) -> Result<Option<String>, std::str::Utf8Error> {
//   if s.is_null() {
//     Ok(None)
//   } else {
//     c_str_to_string(s).map(Some)
//   }
// }
//
//
// impl TryInto<Paginator> for FFIPaginator {
//   type Error = std::str::Utf8Error;
//   fn try_into(self) -> Result<Paginator, Self::Error> {
//     Ok(
//       Paginator {
//         start_time: match self.start_time { 0 => None, v => Some(v) },
//         end_time: match self.end_time { 0 => None, v => Some(v) },
//         limit: match self.limit { 0 => None, v => Some(v) },
//         before: nullable_cstr(self.before)?,
//         after: nullable_cstr(self.after)?,
//       }
//     )
//   }
// }
//
//
// #[derive(Error, Debug)]
// pub enum OpenlimitsSharpError {
//   #[error("Invalid argument {0}")]
//   InvalidArgument(String),
//   #[error("Failed to initialize: {0}")]
//   InitializeException(String),
//   #[error("Failed to subscribe: {0}")]
//   SubscribeException(String),
//   #[error("{0}")]
//   OpenLimitsError(#[from] OpenLimitsError)
// }
//
// #[repr(u32)]
// #[derive(Debug, Copy, Clone)]
// pub enum OpenLimitsResultTag {
//   Ok,
//   InvalidArgument,
//   BinanceError,
//   CoinbaseError,
//   NashProtocolError,
//   MissingImplementation,
//   AssetNotFound,
//   NoApiKeySet,
//   InternalServerError,
//   ServiceUnavailable,
//   Unauthorized,
//   SymbolNotFound,
//   SocketError,
//   GetTimestampFailed,
//   ReqError,
//   InvalidHeaderError,
//   InvalidPayloadSignature,
//   IoError,
//   PoisonError,
//   JsonError,
//   ParseFloatError,
//   UrlParserError,
//   Tungstenite,
//   TimestampError,
//   UnkownResponse,
//   NotParsableResponse,
//   MissingParameter,
//
//   WebSocketMessageNotSupported,
//
//   InitializeException,
//   SubscribeException,
//   NoMarketPair
// }
//
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct OpenLimitsResult {
//   tag: OpenLimitsResultTag,
//   message: *mut c_char
// }
//
// fn result_to_ffi(r: Result<(), OpenlimitsSharpError>) -> OpenLimitsResult {
//   match r {
//     Ok(_) => OpenLimitsResult { tag: OpenLimitsResultTag::Ok, message: std::ptr::null_mut() },
//     Err(e) => {
//       match e {
//         OpenlimitsSharpError::InvalidArgument(msg) => OpenLimitsResult { tag: OpenLimitsResultTag::InvalidArgument, message: string_to_c_str(msg) },
//         OpenlimitsSharpError::InitializeException(msg) => OpenLimitsResult { tag: OpenLimitsResultTag::InitializeException, message: string_to_c_str(msg) },
//         OpenlimitsSharpError::SubscribeException(msg) => OpenLimitsResult { tag: OpenLimitsResultTag::SubscribeException, message: string_to_c_str(msg) },
//         OpenlimitsSharpError::OpenLimitsError(e) => {
//           let message = match &e {
//             OpenLimitsError::BinanceError(e) => e.msg.clone(),
//             OpenLimitsError::CoinbaseError(e) => e.message.clone(),
//             OpenLimitsError::NashProtocolError(e) => e.0.to_string(),
//             OpenLimitsError::MissingImplementation(e) => e.message.clone(),
//             OpenLimitsError::AssetNotFound() => String::from("Asset not found"),
//             OpenLimitsError::NoApiKeySet() => String::from("No api key set"),
//             OpenLimitsError::InternalServerError() => String::from("Internal server error"),
//             OpenLimitsError::ServiceUnavailable() => String::from("Service unavailable"),
//             OpenLimitsError::Unauthorized() => String::from("Unauthorized"),
//             OpenLimitsError::SymbolNotFound() => String::from("Symbol not found"),
//             OpenLimitsError::SocketError() => String::from("Socket error"),
//             OpenLimitsError::GetTimestampFailed() => String::from("Get timestamp failed"),
//             OpenLimitsError::ReqError(e) => e.to_string(),
//             OpenLimitsError::InvalidHeaderError(e) => e.to_string(),
//             OpenLimitsError::InvalidPayloadSignature(e) => e.to_string(),
//             OpenLimitsError::IoError(e) => e.to_string(),
//             OpenLimitsError::PoisonError() => String::from("Poison error"),
//             OpenLimitsError::JsonError(e) => e.to_string(),
//             OpenLimitsError::ParseFloatError(e) => e.to_string(),
//             OpenLimitsError::UrlParserError(e) => e.to_string(),
//             OpenLimitsError::Tungstenite(e) => e.to_string(),
//             OpenLimitsError::TimestampError(e) => e.to_string(),
//             OpenLimitsError::UnkownResponse(e) => e.clone(),
//             OpenLimitsError::NotParsableResponse(e) => e.clone(),
//             OpenLimitsError::MissingParameter(e) => e.clone(),
//             OpenLimitsError::WebSocketMessageNotSupported() => String::from("WebSocket message not supported"),
//             OpenLimitsError::NoMarketPair => String::from("No market pair")
//           };
//           let tag = match &e {
//             OpenLimitsError::BinanceError(_) => OpenLimitsResultTag::BinanceError,
//             OpenLimitsError::CoinbaseError(_) => OpenLimitsResultTag::CoinbaseError,
//             OpenLimitsError::NashProtocolError(_) => OpenLimitsResultTag::NashProtocolError,
//             OpenLimitsError::MissingImplementation(_) => OpenLimitsResultTag::MissingImplementation,
//             OpenLimitsError::AssetNotFound() => OpenLimitsResultTag::AssetNotFound,
//             OpenLimitsError::NoApiKeySet() => OpenLimitsResultTag::NoApiKeySet,
//             OpenLimitsError::InternalServerError() => OpenLimitsResultTag::InternalServerError,
//             OpenLimitsError::ServiceUnavailable() => OpenLimitsResultTag::ServiceUnavailable,
//             OpenLimitsError::Unauthorized() => OpenLimitsResultTag::Unauthorized,
//             OpenLimitsError::SymbolNotFound() => OpenLimitsResultTag::SymbolNotFound,
//             OpenLimitsError::SocketError() => OpenLimitsResultTag::SocketError,
//             OpenLimitsError::GetTimestampFailed() => OpenLimitsResultTag::GetTimestampFailed,
//             OpenLimitsError::ReqError(_) => OpenLimitsResultTag::ReqError,
//             OpenLimitsError::InvalidHeaderError(_) => OpenLimitsResultTag::InvalidHeaderError,
//             OpenLimitsError::InvalidPayloadSignature(_) => OpenLimitsResultTag::InvalidPayloadSignature,
//             OpenLimitsError::IoError(_) => OpenLimitsResultTag::IoError,
//             OpenLimitsError::PoisonError() => OpenLimitsResultTag::PoisonError,
//             OpenLimitsError::JsonError(_) => OpenLimitsResultTag::JsonError,
//             OpenLimitsError::ParseFloatError(_) => OpenLimitsResultTag::ParseFloatError,
//             OpenLimitsError::UrlParserError(_) => OpenLimitsResultTag::UrlParserError,
//             OpenLimitsError::Tungstenite(_) => OpenLimitsResultTag::Tungstenite,
//             OpenLimitsError::TimestampError(_) => OpenLimitsResultTag::TimestampError,
//             OpenLimitsError::UnkownResponse(_) => OpenLimitsResultTag::UnkownResponse,
//             OpenLimitsError::NotParsableResponse(_) => OpenLimitsResultTag::NotParsableResponse,
//             OpenLimitsError::MissingParameter(_) => OpenLimitsResultTag::MissingParameter,
//             OpenLimitsError::WebSocketMessageNotSupported() => OpenLimitsResultTag::WebSocketMessageNotSupported,
//             OpenLimitsError::NoMarketPair => OpenLimitsResultTag::NoMarketPair,
//           };
//           OpenLimitsResult { tag, message: string_to_c_str(message) }
//         },
//       }
//     }
//   }
// }
//
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct FFIAskBid {
//   pub price: *mut c_char,
//   pub qty: *mut c_char,
// }
//
// #[repr(u32)]
// #[derive(Debug, Copy, Clone)]
// pub enum FFILiquidity {
//   Unknown,
//   Maker,
//   Taker,
// }
// #[repr(u32)]
// #[derive(Debug, Copy, Clone)]
// pub enum FFISide {
//   Buy,
//   Sell,
// }
//
// #[repr(u32)]
// #[derive(Debug, Copy, Clone)]
// pub enum FFITIF {
//   GTC,
//   FOK,
//   IOC,
//   GTT
// }
//
// fn ffitif_to_tif(tif: FFITIF, ms: u64) -> TimeInForce {
//   match tif {
//     FFITIF::GTC => TimeInForce::GoodTillCancelled,
//     FFITIF::IOC => TimeInForce::ImmediateOrCancelled,
//     FFITIF::FOK => TimeInForce::FillOrKill,
//     FFITIF::GTT => TimeInForce::GoodTillTime(
//       Duration::milliseconds(ms as i64)
//     ),
//   }
// }
//
// #[repr(u32)]
// #[derive(Debug, Copy, Clone)]
// pub enum FFIOrderType {
//   Limit,
//   Market,
//   StopLimit,
//   StopMarket,
//   Unknown,
// }
//
// fn order_type_to_ffi(t: OrderType) -> FFIOrderType {
//   match t {
//     OrderType::Limit => FFIOrderType::Limit,
//     OrderType::Market => FFIOrderType::Market,
//     OrderType::StopLimit => FFIOrderType::StopLimit,
//     OrderType::StopMarket => FFIOrderType::StopMarket,
//     OrderType::Unknown => FFIOrderType::Unknown,
//   }
// }
//
// #[repr(u32)]
// #[derive(Debug, Copy, Clone)]
// pub enum FFIOrderStatus {
//   New,
//   PartiallyFilled,
//   Filled,
//   Canceled,
//   PendingCancel,
//   Rejected,
//   Expired,
//   Open,
//   Pending,
//   Active,
// }
//
//
// fn order_status_to_ffi(t: OrderStatus) -> FFIOrderStatus {
//   match t {
//     OrderStatus::New => FFIOrderStatus::New,
//     OrderStatus::PartiallyFilled => FFIOrderStatus::PartiallyFilled,
//     OrderStatus::Filled => FFIOrderStatus::Filled,
//     OrderStatus::Canceled => FFIOrderStatus::Canceled,
//     OrderStatus::PendingCancel => FFIOrderStatus::PendingCancel,
//     OrderStatus::Rejected => FFIOrderStatus::Rejected,
//     OrderStatus::Expired => FFIOrderStatus::Expired,
//     OrderStatus::Open => FFIOrderStatus::Open,
//     OrderStatus::Pending => FFIOrderStatus::Pending,
//     OrderStatus::Active => FFIOrderStatus::Active,
//   }
// }
//
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct FFITrade {
//   id: *mut c_char,
//   buyer_order_id: *mut c_char,
//   seller_order_id: *mut c_char,
//   market_pair: *mut c_char,
//   price: *mut c_char,
//   qty: *mut c_char,
//   fees: *mut c_char,
//   side: FFISide,
//   liquidity: FFILiquidity,
//   created_at: u64,
// }
//
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct FFIOrder {
//   pub id: *mut c_char,
//   pub market_pair: *mut c_char,
//   pub client_order_id: *mut c_char,
//   pub created_at: u64,
//   pub order_type: FFIOrderType,
//   pub side: FFISide,
//   pub status: FFIOrderStatus,
//   pub size: *mut c_char,
//   pub price: *mut c_char,
//   pub remaining: *mut c_char,
// }
//
// fn order_to_ffi(t: Order) -> FFIOrder {
//   FFIOrder {
//     id: string_to_c_str(t.id),
//     market_pair: string_to_c_str(t.market_pair),
//     client_order_id: match t.client_order_id {
//       None => std::ptr::null_mut(),
//       Some(client_order_id) => string_to_c_str(client_order_id)
//     },
//     created_at: match t.created_at {
//       None => 0,
//       Some(created_at) => created_at
//     },
//     order_type: order_type_to_ffi(t.order_type),
//     side: match t.side {
//       Side::Buy => FFISide::Buy,
//       Side::Sell => FFISide::Sell,
//     },
//     status: order_status_to_ffi(t.status),
//     size: string_to_c_str(t.size.to_string()),
//     price: match t.price {
//       Some(price) => string_to_c_str(price.to_string()),
//       None => std::ptr::null_mut()
//     },
//     remaining: match t.remaining {
//       Some(rem) =>  string_to_c_str(rem.to_string()),
//       None => std::ptr::null_mut()
//     }
//   }
// }
//
// #[repr(C)]
// #[derive(Debug, Copy, Clone)]
// pub struct FFIGetHistoricTradesRequest {
//   market: *mut c_char,
//   paginator: *mut FFIPaginator
// }
//
//
// fn to_ffi_ask_bid(f: &AskBid) -> FFIAskBid {
//   FFIAskBid {
//     price: string_to_c_str(f.price.to_string()),
//     qty: string_to_c_str(f.qty.to_string())
//   }
// }
//
// fn to_ffi_candle(f: &Candle) -> FFICandle {
//   FFICandle {
//     time: f.time,
//     low: f.low.to_f64().unwrap(),
//     high: f.high.to_f64().unwrap(),
//     open: f.open.to_f64().unwrap(),
//     close: f.close.to_f64().unwrap(),
//     volume: f.volume.to_f64().unwrap(),
//   }
// }
//
// fn option_string_to_c_str(s: Option<String>) -> *mut c_char {
//   match s {
//     None => std::ptr::null_mut(),
//     Some(s ) => string_to_c_str(s)
//   }
// }
//
// fn to_ffi_trade(f: &Trade) -> FFITrade {
//   FFITrade {
//     id: string_to_c_str(f.id.clone()),
//     buyer_order_id: option_string_to_c_str(f.buyer_order_id.clone()),
//     seller_order_id: option_string_to_c_str(f.seller_order_id.clone()),
//     market_pair: string_to_c_str(f.market_pair.clone()),
//     price: string_to_c_str(f.price.to_string()),
//     qty: string_to_c_str(f.qty.to_string()),
//     fees: match f.fees {
//       Some(f) => string_to_c_str(f.to_string()),
//       None => std::ptr::null_mut(),
//     },
//     side: match f.side {
//       Side::Buy => FFISide::Buy,
//       Side::Sell => FFISide::Sell,
//     },
//     liquidity: match f.liquidity {
//       Some(Liquidity::Maker) => FFILiquidity::Maker,
//       Some(Liquidity::Taker) => FFILiquidity::Taker,
//       None => FFILiquidity::Unknown,
//     },
//     created_at: f.created_at,
//   }
// }
//
// #[repr(C)]
// #[derive(Debug)]
// pub struct FFIBinanceConfig {
//     apikey: *mut c_char,
//     secret: *mut c_char,
//     sandbox: bool
// }
//
// type Out<T> = *mut T;
//
//
// fn binance_credentials_from_ptrs(apikey: *mut c_char, secret: *mut c_char) -> Result<Option<BinanceCredentials>, std::str::Utf8Error> {
//   if apikey.is_null() {
//     return Ok(None)
//   }
//   if secret.is_null() {
//     return Ok(None)
//   }
//
//   Ok(
//     Some(
//       BinanceCredentials {
//         api_key: c_str_to_string(apikey)?,
//         api_secret: c_str_to_string(secret)?
//       }
//     )
//   )
// }
//
// impl TryInto<BinanceParameters> for FFIBinanceConfig {
//   type Error = ();
//   fn try_into(self) -> Result<BinanceParameters, Self::Error> {
//     Ok(
//       BinanceParameters {
//         credentials: binance_credentials_from_ptrs(self.apikey, self.secret).map_err(|_|())?,
//         sandbox: self.sandbox,
//       }
//     )
//   }
// }
//
// #[repr(u32)]
// #[derive(Debug)]
// pub enum FFINashEnv {
//   Sandbox,
//   Production
// }
//
// #[repr(C)]
// pub struct ExchangeClient {
//   client: AnyExchange,
//   init_params: InitAnyExchange,
//   channel: Option<tokio::sync::mpsc::UnboundedSender<SubthreadCmd>>,
//   runtime: tokio::runtime::Runtime
// }
//
// #[repr(C)]
// #[derive(Debug)]
// pub struct InitResult {
//   client: *mut ExchangeClient,
// }
// type SubResult = std::result::Result<openlimits::exchange_ws::CallbackHandle, openlimits::errors::OpenLimitsError>;
// type SubChannel = tokio::sync::oneshot::Sender<SubResult>;
// pub enum SubthreadCmd {
//   Sub(Subscription, SubChannel),
//   Disconnect
// }
//
// #[no_mangle]
// pub  extern "cdecl" fn init_binance(
//   config: FFIBinanceConfig,
//   out_client: Out<*mut ExchangeClient>
// ) -> OpenLimitsResult {
//   let call = move|| -> Result<(), OpenlimitsSharpError>{
//     let init_params: InitAnyExchange = config.try_into().map(InitAnyExchange::Binance).map_err(|_| OpenlimitsSharpError::InitializeException(String::from("Failed to parse config")))?;
//     let mut runtime = tokio::runtime::Builder::new().basic_scheduler().enable_all().build().map_err(|_| OpenlimitsSharpError::InitializeException(String::from("Failed to start tokio runtime")))?;
//
//     let client_future = OpenLimits::instantiate(init_params.clone());
//     let client: AnyExchange = runtime.block_on(client_future)?;
//
//
//     let b = Box::new(ExchangeClient{
//       client,
//       init_params,
//       channel: None,
//       runtime
//     });
//     unsafe {
//       *out_client = Box::into_raw(b);
//       Ok(())
//     }
//   };
//
//   result_to_ffi(call())
// }
//
// #[no_mangle]
// pub  extern "cdecl" fn init_coinbase(
//   apikey: *mut c_char,
//   api_secret: *mut c_char,
//   passphrase: *mut c_char,
//   sandbox: bool,
//   out_client: Out<*mut ExchangeClient>
// ) -> OpenLimitsResult {
//   let call = move|| -> Result<(), OpenlimitsSharpError>{
//     let api_key = nullable_cstr(apikey).map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse apikey string. Invalid character on pos {}", e.valid_up_to()))
//     )?;
//
//     let api_secret = nullable_cstr(api_secret).map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse api_secret string. Invalid character on pos {}", e.valid_up_to()))
//     )?;
//
//     let passphrase = nullable_cstr(passphrase).map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse passphrase string. Invalid character on pos {}", e.valid_up_to()))
//     )?;
//
//     let init_params: InitAnyExchange = InitAnyExchange::Coinbase(
//       CoinbaseParameters {
//         sandbox,
//         credentials: match (api_key, api_secret, passphrase) {
//           (Some(api_key), Some(api_secret), Some(passphrase)) => Ok(
//             Some(
//               CoinbaseCredentials {
//                 api_key,
//                 api_secret,
//                 passphrase
//               }
//             )
//           ),
//           (None, None, None) => Ok(None),
//           _ => Err(OpenlimitsSharpError::InvalidArgument(format!("Invalid credentials")))
//         }?
//       }
//     );
//
//     let mut runtime = tokio::runtime::Builder::new().basic_scheduler().enable_all().build().map_err(|_| OpenlimitsSharpError::InitializeException(String::from("Failed to start tokio runtime")))?;
//
//     let client_future = OpenLimits::instantiate(init_params.clone());
//     let client: AnyExchange = runtime.block_on(client_future)?;
//
//
//     let b = Box::new(ExchangeClient{
//       client,
//       init_params,
//       channel: None,
//       runtime
//     });
//     unsafe {
//       *out_client = Box::into_raw(b);
//       Ok(())
//     }
//   };
//
//   result_to_ffi(call())
// }
//
//
//
// #[no_mangle]
// pub  extern "cdecl" fn init_nash(
//   apikey: *mut c_char,
//   secret: *mut c_char,
//   client_id: u64,
//   environment: FFINashEnv,
//   timeout: u64,
//   affiliate_code: *mut c_char,
//   out_client: Out<*mut ExchangeClient>
// )  -> OpenLimitsResult {
//   let call = move|| -> Result<(), OpenlimitsSharpError>{
//     let mut credentials: Option<NashCredentials> = None;
//     if !apikey.is_null() && !secret.is_null() {
//       credentials = Some(
//         NashCredentials {
//           secret: c_str_to_string(secret).map_err(|e|
//             OpenlimitsSharpError::InvalidArgument(format!("Failed to parse market string. Invalid character on pos {}", e.valid_up_to()))
//           )?,
//           session: c_str_to_string(apikey).map_err(|e|
//             OpenlimitsSharpError::InvalidArgument(format!("Failed to parse session string. Invalid character on pos {}", e.valid_up_to()))
//           )?
//         }
//       )
//     }
//
//     let environment = match environment {
//       FFINashEnv::Production => Environment::Production,
//       FFINashEnv::Sandbox => Environment::Sandbox,
//     };
//
//     let affiliate_code = nullable_cstr(affiliate_code).map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse affiliate_code string. Invalid character on pos {}", e.valid_up_to()))
//     )?;
//
//     let nash_params =  NashParameters {
//       affiliate_code,
//       credentials,
//       client_id,
//       timeout: std::time::Duration::from_millis(timeout),
//       environment
//     };
//
//     let init_params = InitAnyExchange::Nash(
//       nash_params
//     );
//
//     let mut runtime = tokio::runtime::Builder::new().basic_scheduler().enable_all().build().map_err(|_| OpenlimitsSharpError::InitializeException(String::from("Failed to start tokio runtime")))?;
//
//     let client_future = OpenLimits::instantiate(init_params.clone());
//     let client: AnyExchange = runtime.block_on(client_future)?;
//
//     let b = Box::new(ExchangeClient{
//       client,
//       init_params,
//       channel: None,
//       runtime
//     });
//     unsafe {
//       *out_client = Box::into_raw(b);
//       Ok(())
//     }
//   };
//
//   result_to_ffi(call())
// }
//
// #[no_mangle]
// pub  extern "cdecl" fn order_book(
//   client: *mut ExchangeClient,
//   market: *mut c_char,
//   bids_buff: *mut FFIAskBid, bids_buff_len: u64, actual_bids_buff_len: Out<u64>,
//   asks_buff: *mut FFIAskBid, asks_buff_len: u64, actual_asks_buff_len: Out<u64>,
//   last_update_id: Out<u64>,
//   update_id: Out<u64>,
// ) -> OpenLimitsResult {
//   let call = move|| -> Result<(), OpenlimitsSharpError>{
//     if client.is_null() {
//       return Err(OpenlimitsSharpError::InvalidArgument(String::from("client is null")));
//     }
//
//     if market.is_null() {
//       return Err(OpenlimitsSharpError::InvalidArgument(String::from("market is null")));
//     }
//     let market_pair = c_str_to_string(market).map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse market string. Invalid character on pos {}", e.valid_up_to()))
//     )?;
//
//     let req = OrderBookRequest {
//       market_pair
//     };
//     unsafe {
//       let resp = (*client).runtime.block_on(
//         (*client).client.order_book(&req)
//       )?;
//
//       let bids = std::slice::from_raw_parts_mut::<FFIAskBid>(bids_buff, bids_buff_len as usize);
//       let ffi_bids: Vec<FFIAskBid> = resp.bids.iter().map(to_ffi_ask_bid).collect();
//       let l = std::cmp::min(bids_buff_len as usize, ffi_bids.len() as usize);
//       bids[0..l].copy_from_slice(&ffi_bids[0..l]);
//       (*actual_bids_buff_len) = l as u64;
//
//       let asks = std::slice::from_raw_parts_mut::<FFIAskBid>(asks_buff, asks_buff_len as usize);
//       let ffi_asks: Vec<FFIAskBid> = resp.asks.iter().map(to_ffi_ask_bid).collect();
//       let l = std::cmp::min(asks_buff_len as usize, ffi_asks.len() as usize);
//       asks[0..l].copy_from_slice(&ffi_asks[0..l]);
//       (*actual_asks_buff_len) = l as u64;
//       (*last_update_id) = resp.last_update_id.unwrap_or_default();
//       (*update_id) = resp.update_id.unwrap_or_default();
//     };
//     Ok(())
//   };
//
//   result_to_ffi(call())
// }
//
//
//
// #[no_mangle]
// pub  extern "cdecl" fn get_price_ticker(
//   client: *mut ExchangeClient,
//   market: *mut c_char,
//   price: Out<f64>
// ) -> OpenLimitsResult {
//   let call = move|| -> Result<(), OpenlimitsSharpError> {
//     if client.is_null() {
//       return Err(OpenlimitsSharpError::InvalidArgument(String::from("client is null")));
//     }
//
//     if market.is_null() {
//       return Err(OpenlimitsSharpError::InvalidArgument(String::from("market is null")));
//     }
//     let market_pair = c_str_to_string(market).map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse market string. Invalid character on pos {}", e.valid_up_to()))
//     )?;
//
//     let req = GetPriceTickerRequest {
//       market_pair
//     };
//     unsafe {
//       let resp = (*client).runtime.block_on(
//         (*client).client.get_price_ticker(&req)
//       )?;
//       let price_opt = resp.price;
//       let price_opt = price_opt.map(|f| f.to_f64()).flatten();
//       (*price) = price_opt.unwrap_or(std::f64::NAN);
//       Ok(())
//     }
//   };
//
//
//   result_to_ffi(call())
// }
//
//
// #[no_mangle]
// pub  extern "cdecl" fn get_historic_rates(
//   client: *mut ExchangeClient,
//   market: *mut c_char,
//   interval: FFIInterval,
//   paginator: *mut FFIPaginator,
//   candles_buff: *mut FFICandle, candles_buff_len: usize, actual_candles_buff_len: Out<usize>,
// ) -> OpenLimitsResult {
//   let call = move|| -> Result<(), OpenlimitsSharpError> {
//     if client.is_null() {
//       return Err(OpenlimitsSharpError::InvalidArgument(String::from("client is null")));
//     }
//     let mut paginator_res: Option<Result<Paginator, _>> = None;
//     if !paginator.is_null() {
//       unsafe {
//         let pag: Result<Paginator, _> = (*paginator).try_into();
//         paginator_res = Some(pag);
//       }
//     }
//     let paginator = paginator_res.transpose().map_err(|_| OpenlimitsSharpError::InvalidArgument(String::from("Invalid paginator")))?;
//     let market_pair = c_str_to_string(market).map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse market string. Invalid character on pos {}", e.valid_up_to()))
//     )?;
//     let interval = interval_from_ffi_interval(interval).map_err(|_| OpenlimitsSharpError::InvalidArgument(String::from("Invalid interval")))?;
//
//     let req = GetHistoricRatesRequest {
//       paginator,
//       market_pair,
//       interval
//     };
//     unsafe {
//       let resp = (*client).runtime.block_on(
//         (*client).client.get_historic_rates(&req)
//       )?;
//
//
//       let canles = std::slice::from_raw_parts_mut::<FFICandle>(candles_buff, candles_buff_len);
//       let ffi_candles: Vec<FFICandle> = resp.iter().map(to_ffi_candle).collect();
//       let l = std::cmp::min(candles_buff_len, ffi_candles.len());
//       canles[0..l].copy_from_slice(&ffi_candles[0..l]);
//       (*actual_candles_buff_len) = l;
//       Ok(())
//     }
//   };
//   result_to_ffi(call())
// }
//
// #[no_mangle]
// pub  extern "cdecl" fn get_historic_trades(
//   client: *mut ExchangeClient,
//   market: *mut c_char,
//   paginator: *mut FFIPaginator,
//   buff: *mut FFITrade, buff_len: usize, actual_buff_len: Out<usize>,
// ) -> OpenLimitsResult {
//   let call = move|| -> Result<(), OpenlimitsSharpError> {
//     if client.is_null() {
//       return Err(OpenlimitsSharpError::InvalidArgument(String::from("client is null")));
//     }
//     let market_pair = c_str_to_string(market).map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse market string. Invalid character on pos {}", e.valid_up_to()))
//     )?;
//
//
//     let mut paginator_res: Option<Result<Paginator, _>> = None;
//     if !paginator.is_null() {
//       unsafe {
//         let pag: Result<Paginator, _> = (*paginator).try_into();
//         paginator_res = Some(pag);
//       }
//     }
//     let paginator = paginator_res.transpose().map_err(|_| OpenlimitsSharpError::InvalidArgument(String::from("Invalid paginator")))?;
//
//     let req = GetHistoricTradesRequest {
//       paginator,
//       market_pair,
//     };
//     unsafe {
//       let resp = (*client).runtime.block_on(
//         (*client).client.get_historic_trades(&req)
//       )?;
//
//       let trades = std::slice::from_raw_parts_mut::<FFITrade>(buff, buff_len);
//       let ffi_trades: Vec<FFITrade> = resp.iter().map(to_ffi_trade).collect();
//       let l = std::cmp::min(buff_len, ffi_trades.len());
//       trades[0..l].copy_from_slice(&ffi_trades[0..l]);
//       (*actual_buff_len) = l;
//       Ok(())
//     }
//   };
//   result_to_ffi(call())
// }
// #[no_mangle]
//
// pub extern "cdecl" fn place_order(
//   client: *mut ExchangeClient,
//   market: *mut c_char,
//   qty: *mut c_char,
//   limit: bool,
//   price: *mut c_char,
//   side: FFISide,
//   tif: FFITIF,
//   tif_duration: u64,
//   _post_only: bool,
//
//   result: Out<FFIOrder>
// ) -> OpenLimitsResult {
//   let call = move|| -> Result<(), OpenlimitsSharpError> {
//     if client.is_null() {
//       return Err(OpenlimitsSharpError::InvalidArgument(String::from("client is null")));
//     }
//     let market_pair = c_str_to_string(market).map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse market string. Invalid character on pos {}", e.valid_up_to()))
//     )?;
//     let size = c_str_to_string(qty).map(|q| Decimal::from_str(q.as_str()));
//     let size = size.map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse size string. Invalid character on pos {}", e.valid_up_to()))
//     )?;
//     let size = size.map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse size string: {}", e))
//     )?;
//
//
//     if limit == false {
//       let  req = OpenMarketOrderRequest {
//         market_pair,
//         size
//       };
//
//       unsafe {
//         #[allow(unreachable_patterns)]
//         match side {
//           FFISide::Buy => {
//             let order = (*client).runtime.block_on(
//               (*client).client.market_buy(&req)
//             )?;
//             (*result) = order_to_ffi(order);
//             return Ok(());
//           },
//           FFISide::Sell => {
//             let order = (*client).runtime.block_on(
//               (*client).client.market_sell(&req)
//             )?;
//             (*result) = order_to_ffi(order);
//             return Ok(());
//           },
//           e => return Err(OpenlimitsSharpError::InvalidArgument(format!("Invalid side size string: {:?}", e)))
//         }
//       }
//     }
//     let price = c_str_to_string(price).map(|q| Decimal::from_str(q.as_str())).map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse price string. Invalid character on pos {}", e.valid_up_to()))
//     )?;
//     let price = price.map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse price string: {}", e))
//     )?;
//
//     let time_in_force = ffitif_to_tif(tif, tif_duration);
//     let req = OpenLimitOrderRequest {
//       market_pair,
//       price,
//       time_in_force,
//       size,
//       post_only: _post_only
//     };
//     unsafe {
//       #[allow(unreachable_patterns)]
//       match side {
//         FFISide::Buy => {
//           let order = (*client).runtime.block_on(
//             (*client).client.limit_buy(&req)
//           )?;
//           (*result) = order_to_ffi(order);
//           return Ok(());
//         },
//         FFISide::Sell => {
//           let order = (*client).runtime.block_on(
//             (*client).client.limit_sell(&req)
//           )?;
//           (*result) = order_to_ffi(order);
//           return Ok(());
//         },
//         e => return Err(OpenlimitsSharpError::InvalidArgument(format!("Invalid side size string: {:?}", e)))
//       }
//     }
//   };
//
//   result_to_ffi(call())
// }
//
// #[no_mangle]
// pub  extern "cdecl" fn get_all_open_orders(
//   client: *mut ExchangeClient,
//   buff: *mut FFIOrder, buff_len: usize, actual_buff_len: Out<usize>,
// ) -> OpenLimitsResult {
//   let call = move|| -> Result<(), OpenlimitsSharpError> {
//     if client.is_null() {
//       return Err(OpenlimitsSharpError::InvalidArgument(String::from("client is null")));
//     }
//
//     unsafe {
//       let resp = (*client).runtime.block_on(
//         (*client).client.get_all_open_orders()
//       )?;
//
//       let orders = std::slice::from_raw_parts_mut::<FFIOrder>(buff, buff_len);
//       let ffi_orders: Vec<FFIOrder> = resp.into_iter().map(order_to_ffi).collect();
//       let l = std::cmp::min(buff_len, ffi_orders.len());
//       orders[0..ffi_orders.len()].copy_from_slice(&ffi_orders[0..l]);
//       (*actual_buff_len) = l;
//     };
//     Ok(())
//   };
//
//   result_to_ffi(call())
// }
//
// #[no_mangle]
// pub  extern "cdecl" fn get_order_history(
//   client: *mut ExchangeClient,
//   market: *mut c_char,
//   paginator: *mut FFIPaginator,
//   buff: *mut FFIOrder, buff_len: usize, actual_buff_len: Out<usize>,
// ) -> OpenLimitsResult {
//   let call = move|| -> Result<(), OpenlimitsSharpError> {
//     if client.is_null() {
//       return Err(OpenlimitsSharpError::InvalidArgument(String::from("client is null")));
//     }
//     let market_pair = nullable_cstr(market).map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse market string. Invalid character on pos {}", e.valid_up_to()))
//     )?;
//
//
//     let mut paginator_res: Option<Result<Paginator, _>> = None;
//     if !paginator.is_null() {
//       unsafe {
//         let pag: Result<Paginator, _> = (*paginator).try_into();
//         paginator_res = Some(pag);
//       }
//     }
//     let paginator = paginator_res.transpose().map_err(|_| OpenlimitsSharpError::InvalidArgument(String::from("Invalid paginator")))?;
//
//     let req = GetOrderHistoryRequest {
//       paginator,
//       market_pair,
//     };
//     unsafe {
//       let resp = (*client).runtime.block_on(
//         (*client).client.get_order_history(&req)
//       )?;
//
//       let orders = std::slice::from_raw_parts_mut::<FFIOrder>(buff, buff_len);
//       let ffi_orders: Vec<FFIOrder> = resp.into_iter().map(order_to_ffi).collect();
//       let l = std::cmp::min(buff_len, ffi_orders.len());
//
//       orders[0..l].copy_from_slice(&ffi_orders[0..l]);
//       (*actual_buff_len) = l;
//     }
//     Ok(())
//   };
//
//   result_to_ffi(call())
// }
//
//
//
// #[no_mangle]
// pub  extern "cdecl" fn get_trade_history(
//   client: *mut ExchangeClient,
//   market: *mut c_char,
//   order_id: *mut c_char,
//   paginator: *mut FFIPaginator,
//   buff: *mut FFITrade, buff_len: usize, actual_buff_len: Out<usize>,
// ) -> OpenLimitsResult {
//   let call = move|| -> Result<(), OpenlimitsSharpError> {
//     if client.is_null() {
//       return Err(OpenlimitsSharpError::InvalidArgument(String::from("client is null")));
//     }
//     let market_pair = nullable_cstr(market).map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse market string. Invalid character on pos {}", e.valid_up_to()))
//     )?;
//     let order_id = nullable_cstr(order_id).map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse order_id string. Invalid character on pos {}", e.valid_up_to()))
//     )?;
//
//
//     let mut paginator_res: Option<Result<Paginator, _>> = None;
//     if !paginator.is_null() {
//       unsafe {
//         let pag: Result<Paginator, _> = (*paginator).try_into();
//         paginator_res = Some(pag);
//       }
//     }
//     let paginator = paginator_res.transpose().map_err(|_| OpenlimitsSharpError::InvalidArgument(String::from("Invalid paginator")))?;
//
//     let req = TradeHistoryRequest {
//       paginator,
//       order_id,
//       market_pair,
//     };
//     unsafe {
//       let resp = (*client).runtime.block_on(
//         (*client).client.get_trade_history(&req)
//       )?;
//
//       let trades = std::slice::from_raw_parts_mut::<FFITrade>(buff, buff_len);
//       let ffi_trades: Vec<FFITrade> = resp.iter().map(to_ffi_trade).collect();
//       let l = std::cmp::min(buff_len, ffi_trades.len());
//
//       trades[0..ffi_trades.len()].copy_from_slice(&ffi_trades[0..l]);
//       (*actual_buff_len) = l;
//     }
//     Ok(())
//   };
//
//   result_to_ffi(call())
//
// }
//
//
// #[no_mangle]
// pub  extern "cdecl" fn get_account_balances(
//   client: *mut ExchangeClient,
//   paginator: *mut FFIPaginator,
//   buff: *mut FFIBalance, buff_len: usize, actual_buff_len: Out<usize>,
// ) -> OpenLimitsResult {
//   let call = move|| -> Result<(), OpenlimitsSharpError> {
//     if client.is_null() {
//       return Err(OpenlimitsSharpError::InvalidArgument(String::from("client is null")));
//     }
//
//     let mut paginator_res: Option<Result<Paginator, _>> = None;
//     if !paginator.is_null() {
//       unsafe {
//         let pag: Result<Paginator, _> = (*paginator).try_into();
//         paginator_res = Some(pag);
//       }
//     }
//     let paginator = paginator_res.transpose().map_err(|_| OpenlimitsSharpError::InvalidArgument(String::from("Invalid paginator")))?;
//
//
//     unsafe {
//       let resp = (*client).runtime.block_on(
//         (*client).client.get_account_balances(paginator)
//       )?;
//
//       let balances = std::slice::from_raw_parts_mut::<FFIBalance>(buff, buff_len);
//       let ffi_balances: Vec<FFIBalance> = resp.into_iter().map(to_ffi_balance).collect();
//       let l = std::cmp::min(buff_len, ffi_balances.len());
//
//       balances[0..l].copy_from_slice(&ffi_balances[0..l]);
//       (*actual_buff_len) = l;
//     }
//     Ok(())
//   };
//
//   result_to_ffi(call())
// }
//
//
// #[no_mangle]
// pub  extern "cdecl" fn cancel_all_orders(
//   client: *mut ExchangeClient,
//   market: *mut c_char,
//   buff: *mut *mut c_char, buff_len: usize, actual_buff_len: Out<usize>,
// ) -> OpenLimitsResult {
//   let call = move|| -> Result<(), OpenlimitsSharpError> {
//     if client.is_null() {
//       return Err(OpenlimitsSharpError::InvalidArgument(String::from("client is null")));
//     }
//     let market_pair = nullable_cstr(market).map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse market string. Invalid character on pos {}", e.valid_up_to()))
//     )?;
//
//
//     unsafe {
//       let resp = (*client).runtime.block_on(
//         (*client).client.cancel_all_orders(&CancelAllOrdersRequest {
//           market_pair
//         })
//       )?;
//
//       let ids = std::slice::from_raw_parts_mut::<*mut c_char>(buff, buff_len);
//       let ffi_ids: Vec<*mut c_char> = resp.into_iter().map(|c|string_to_c_str(c.id)).collect();
//       let l = std::cmp::min(buff_len, ffi_ids.len());
//
//       ids[0..l].copy_from_slice(&ffi_ids[0..l]);
//       (*actual_buff_len) = l;
//     }
//     Ok(())
//   };
//   result_to_ffi(call())
// }
//
// #[no_mangle]
// pub extern "cdecl" fn get_order(
//   client: *mut ExchangeClient,
//   order_id: *mut c_char,
//   market: *mut c_char,
//   result: Out<FFIOrder>,
// ) -> OpenLimitsResult {
//   let call = move|| -> Result<(), OpenlimitsSharpError> {
//     if client.is_null() {
//       return Err(OpenlimitsSharpError::InvalidArgument(String::from("client is null")));
//     }
//
//     let id = c_str_to_string(order_id).map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse market string. Invalid character on pos {}", e.valid_up_to()))
//     )?;
//     let market_pair = nullable_cstr(market).map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse market string. Invalid character on pos {}", e.valid_up_to()))
//     )?;
//
//     unsafe {
//       let order = (*client).runtime.block_on(
//         (*client).client.get_order( &GetOrderRequest {
//           id,
//           market_pair
//         })
//       )?;
//       (*result) = order_to_ffi(order);
//     }
//
//     Ok(())
//   };
//
//   result_to_ffi(call())
// }
//
// #[no_mangle]
// pub  extern "cdecl" fn cancel_order(
//   client: *mut ExchangeClient,
//   order_id: *mut c_char,
//   market: *mut c_char,
// ) -> OpenLimitsResult {
//   let call = move|| -> Result<(), OpenlimitsSharpError> {
//     if client.is_null() {
//       return Err(OpenlimitsSharpError::InvalidArgument(String::from("client is null")));
//     }
//     let id = c_str_to_string(order_id).map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse market string. Invalid character on pos {}", e.valid_up_to()))
//     )?;
//     let market_pair = nullable_cstr(market).map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse market string. Invalid character on pos {}", e.valid_up_to()))
//     )?;
//
//     unsafe {
//       (*client).runtime.block_on(
//         (*client).client.cancel_order(&CancelOrderRequest {
//           id,
//           market_pair
//         })
//       )?;
//     }
//     Ok(())
//   };
//   result_to_ffi(call())
// }
//
//
// #[no_mangle]
// pub  extern "cdecl" fn receive_pairs(
//   client: *mut ExchangeClient,
//   buff: *mut FFIMarketPair, buff_len: usize, actual_buff_len: Out<usize>,
// ) -> OpenLimitsResult {
//   let call = move|| -> Result<(), OpenlimitsSharpError> {
//     if client.is_null() {
//       return Err(OpenlimitsSharpError::InvalidArgument(String::from("client is null")));
//     }
//     unsafe {
//       let pairs = (*client).runtime.block_on(
//         (*client).client.retrieve_pairs()
//       )?;
//
//       let pairs_buff = std::slice::from_raw_parts_mut::<FFIMarketPair>(buff, buff_len);
//       let pairs_ffi: Vec<FFIMarketPair> = pairs.into_iter().map(market_pair_to_ffi).collect();
//       let l = std::cmp::min(buff_len, pairs_ffi.len());
//
//       pairs_buff[0..l].copy_from_slice(&pairs_ffi[0..l]);
//       (*actual_buff_len) = l;
//     }
//     Ok(())
//   };
//   result_to_ffi(call())
// }
//
// #[repr(C)]
// #[derive(Copy, Clone)]
// pub struct FFITradeBox(*mut FFITrade);
// unsafe impl Send for FFITradeBox {}
// unsafe impl Sync for FFITradeBox {}
// #[repr(C)]
// #[derive(Copy, Clone)]
// pub struct FFIAskBidBox(*mut FFIAskBid);
// unsafe impl Send for FFIAskBidBox {}
// unsafe impl Sync for FFIAskBidBox {}
//
// #[no_mangle]
// #[allow(unsafe_code)]
// pub  extern "cdecl" fn init_subscriptions(
//   client: *mut ExchangeClient,
//   on_error: extern fn(),
//   on_ping: extern fn(),
//   on_orderbook: extern fn(bids_len: u64, asks_len: u64, market: *mut c_char, last_update_id: u64, update_id: u64),
//   on_trades: extern fn(buff_len: u64, market: *mut c_char),
//   on_disconnet: extern fn(),
//   bids_buff: FFIAskBidBox, bids_buff_len: usize,
//   asks_buff: FFIAskBidBox, asks_buff_len: usize,
//   trades_buff: FFITradeBox, trades_buff_len: usize,
//   sub_handle: Out<*mut tokio::sync::mpsc::UnboundedSender<SubthreadCmd>>
// ) ->  OpenLimitsResult {
//   let (sub_request_tx, mut sub_rx) = tokio::sync::mpsc::unbounded_channel::<SubthreadCmd>();
//
//   let init_params = unsafe {
//     (*client).init_params.clone()
//   };
//   let (finish_tx, finish_rx) = tokio::sync::oneshot::channel::<Result<(), OpenlimitsSharpError>>();
//
//   std::thread::spawn(move || {
//     let call = move|| -> Result<(tokio::runtime::Runtime, OpenLimitsWs<AnyWsExchange>), OpenlimitsSharpError> {
//       let mut rt = tokio::runtime::Builder::new()
//                 .basic_scheduler()
//                 .enable_all()
//                 .build()
//                 .map_err(|_| OpenlimitsSharpError::InitializeException(String::from("Failed to start tokio runtime")))?;
//       let client: OpenLimitsWs<AnyWsExchange> = rt.block_on(OpenLimitsWs::instantiate(init_params))?;
//
//       Ok((rt, client))
//     };
//
//     let (mut rt, client) = match call() {
//       Ok(e) => e,
//       Err(e) => {
//         finish_tx.send(Err(e)).expect("Failed to communicate result back to main thread");
//         return;
//       }
//     };
//     finish_tx.send(Ok(())).expect("Failed to communicate result back to main thread");
//
//     loop {
//       let subcmd = sub_rx.next();
//       let thread_cmd = rt.block_on(subcmd);
//       match thread_cmd {
//         Some(SubthreadCmd::Disconnect) => {
//           break;
//         },
//         Some(SubthreadCmd::Sub(sub, writer)) => {
//           let result = rt.block_on(client.subscribe(sub.clone(), move |resp| {
//             let out_asks = unsafe { std::slice::from_raw_parts_mut::<FFIAskBid>(asks_buff.0, asks_buff_len) };
//             let out_bids = unsafe { std::slice::from_raw_parts_mut::<FFIAskBid>(bids_buff.0, bids_buff_len) };
//             let resp = match resp {
//               Ok(e) => e,
//               Err(_) => {
//                 on_error();
//                 return
//               }
//             };
//             let resp = match resp {
//               WebSocketResponse::Generic(msg) => msg,
//               _ => {
//                 return;
//               }
//             };
//
//             match resp {
//               OpenLimitsWebSocketMessage::Ping => {
//                 on_ping();
//               },
//               OpenLimitsWebSocketMessage::Trades(trades) => {
//                 let out_trades = unsafe { std::slice::from_raw_parts_mut::<FFITrade>(trades_buff.0, trades_buff_len) };
//                 let market = match sub.clone() {
//                   Subscription::Trades(market) => market,
//                   _ => panic!("Unreachable")
//                 };
//                 for (i, trade) in trades.iter().enumerate() {
//                   out_trades[i] = to_ffi_trade(trade);
//                 }
//                 on_trades(trades.len() as u64, string_to_c_str(market));
//               },
//               OpenLimitsWebSocketMessage::OrderBook(resp) => {
//                 let market = match sub.clone() {
//                   Subscription::OrderBookUpdates(market) => market,
//                   _ => panic!("Unreachable")
//                 };
//                 for (i, bid) in resp.bids.iter().enumerate() {
//                   out_bids[i] = to_ffi_ask_bid(bid);
//                 }
//                 for (i, ask) in resp.asks.iter().enumerate() {
//                   out_asks[i] = to_ffi_ask_bid(ask);
//                 }
//                 on_orderbook(
//                   resp.bids.len() as u64,
//                   resp.asks.len() as u64,
//                   string_to_c_str(market.clone()),
//                   resp.last_update_id.unwrap_or_default(),
//                   resp.update_id.unwrap_or_default()
//                 );
//               },
//               OpenLimitsWebSocketMessage::OrderBookDiff(resp) => {
//                 let market = match sub.clone() {
//                   Subscription::OrderBookUpdates(market) => market,
//                   _ => panic!("Unreachable")
//                 };
//                 for (i, bid) in resp.bids.iter().enumerate() {
//                   out_bids[i] = to_ffi_ask_bid(bid);
//                 }
//                 for (i, ask) in resp.asks.iter().enumerate() {
//                   out_asks[i] = to_ffi_ask_bid(ask);
//                 }
//                 on_orderbook(
//                   resp.bids.len() as u64,
//                   resp.asks.len() as u64,
//                   string_to_c_str(market.clone()),
//                   resp.last_update_id.unwrap_or_default(),
//                   resp.update_id.unwrap_or_default()
//                 );
//               }
//             };
//           }));
//           writer.send(result).expect("Failed to send result back to subcribe call");
//         },
//         None => {}
//       }
//     }
//     on_disconnet();
//   });
//
//   unsafe {
//     let r = match  (*client).runtime.block_on(
//       finish_rx
//     ) {
//       Err(error) => Err(OpenlimitsSharpError::InitializeException(format!("Failed while waiting for subscription thread to intialize: {}", error.to_string()))),
//       Ok(e) => e
//     };
//
//     *sub_handle = Box::into_raw(Box::new(sub_request_tx));
//
//     result_to_ffi(r)
//   }
// }
//
//
// #[no_mangle]
// pub extern fn free_string(s: *mut c_char) {
//     unsafe {
//         if s.is_null() { return }
//         CString::from_raw(s)
//     };
// }
//
// #[no_mangle]
// pub  extern "cdecl" fn subscribe_orderbook(
//   client: *mut ExchangeClient,
//   channel: *mut tokio::sync::mpsc::UnboundedSender<SubthreadCmd>,
//   market: *mut c_char,
// ) -> OpenLimitsResult {
//   let call = move|| -> Result<(), OpenlimitsSharpError> {
//     if channel.is_null() {
//       return Err(OpenlimitsSharpError::InvalidArgument(String::from("channel is null")));
//     }
//     let market_pair = c_str_to_string(market).map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse market string. Invalid character on pos {}", e.valid_up_to()))
//     )?;
//
//     let (finish_tx, finish_rx) = tokio::sync::oneshot::channel::<SubResult>();
//     unsafe {
//       (*channel).send(
//         SubthreadCmd::Sub(Subscription::OrderBookUpdates(
//           market_pair,
//         ), finish_tx)
//       ).map_err(|_| OpenlimitsSharpError::SubscribeException(String::from("failed to send subscription to handler")))?;
//
//       let result = (*client).runtime.block_on(finish_rx).map_err(|_| OpenlimitsSharpError::SubscribeException(String::from("failed to get subscription result from handler")))?;
//
//       match result {
//         Ok(_) => Ok(()),
//         Err(e) => Err(OpenlimitsSharpError::OpenLimitsError(e))
//       }
//     }
//   };
//   result_to_ffi(call())
// }
//
// #[no_mangle]
// pub  extern "cdecl" fn subscribe_trades(
//   client: *mut ExchangeClient,
//   channel: *mut tokio::sync::mpsc::UnboundedSender<SubthreadCmd>,
//   market: *mut c_char
// ) -> OpenLimitsResult {
//   let call = move|| -> Result<(), OpenlimitsSharpError> {
//     if channel.is_null() {
//       return Err(OpenlimitsSharpError::InvalidArgument(String::from("channel is null")));
//     }
//     let market_pair = c_str_to_string(market).map_err(|e|
//       OpenlimitsSharpError::InvalidArgument(format!("Failed to parse market string. Invalid character on pos {}", e.valid_up_to()))
//     )?;
//     let (finish_tx, finish_rx) = tokio::sync::oneshot::channel::<SubResult>();
//
//     unsafe {
//       (*channel).send(
//         SubthreadCmd::Sub(Subscription::Trades(
//           market_pair,
//         ), finish_tx)
//       ).map_err(|_| OpenlimitsSharpError::SubscribeException(String::from("failed to send subscription to handler")))?;
//
//       let result = (*client).runtime.block_on(finish_rx).map_err(|_| OpenlimitsSharpError::SubscribeException(String::from("failed to get subscription result from handler")))?;
//
//       match result {
//         Ok(_) => Ok(()),
//         Err(e) => Err(OpenlimitsSharpError::OpenLimitsError(e))
//       }
//     }
//   };
//   result_to_ffi(call())
// }
//
// #[no_mangle]
// pub  extern "cdecl" fn disconnect(
//   channel: *mut tokio::sync::mpsc::UnboundedSender<SubthreadCmd>,
// ) {
//   unsafe {
//     let res = (*channel).send(
//       SubthreadCmd::Disconnect
//     );
//     res.map_err(|_| "Send error").expect("Failed to disconnect");
//   }
// }