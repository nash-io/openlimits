ligen_macro::ignore!();

pub mod symbol;

use symbol::Symbol;

#[derive(Debug, Clone)]
pub struct MarketPair(pub Symbol, pub Symbol);
