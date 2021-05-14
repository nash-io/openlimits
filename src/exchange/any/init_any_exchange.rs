use crate::exchange::coinbase::CoinbaseParameters;
use crate::exchange::nash::NashParameters;
use crate::exchange::binance::BinanceParameters;

/// Exchange parameters, this is used in AnyExchange enum
#[derive(Clone)]
pub enum InitAnyExchange {
    Nash(NashParameters),
    Binance(BinanceParameters),
    Coinbase(CoinbaseParameters),
}