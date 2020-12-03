//! In some contexts, such as bindings in other languages (e.g., Python via pyo3), it is not possible to use trait
//! constraints on generics. This module provides an enum wrapper type for all openlimits exchanges that code can
//! use to operate over any openlimits-supported exchange without generics

use crate::exchange_info::ExchangeInfoRetrieval;
use crate::binance::{Binance, BinanceParameters};
use crate::coinbase::{Coinbase, CoinbaseParameters};
use crate::nash::{Nash, NashParameters};
use crate::exchange::{Exchange, ExchangeAccount, ExchangeMarketData};
use crate::exchange_ws::ExchangeWs;

#[derive(Clone)]
pub enum InitAnyExchange {
    Nash(NashParameters),
    Binance(BinanceParameters),
    Coinbase(CoinbaseParameters)
}

pub trait AnyExchange:
    Exchange + ExchangeInfoRetrieval + ExchangeAccount + ExchangeMarketData
{}
pub trait AnyExchangeWs: ExchangeWs {}

impl AnyExchange for Nash {}
impl AnyExchange for Binance {}
impl AnyExchange for Coinbase {}