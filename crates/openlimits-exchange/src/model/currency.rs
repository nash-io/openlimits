use serde::{Serialize, Deserialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Currency {
    BTC,
    ETH,
    XRP,
    FIL,
    DASH,
    CNY,
    SOL,
    USD,
    USDC,
    USDT,
    BUSD,
    Other(String)
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other(s) => write!(f, "{}", s),
            _ => write!(f, "{:?}", self)
        }
    }
}
