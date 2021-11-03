use rust_decimal::Decimal;

// TODO: Use MarketPair inside MarketPairInfo.

#[derive(Debug, Clone)]
pub struct MarketPairInfo {
    pub base: String,
    pub quote: String,
    pub symbol: String,
    pub base_increment: Decimal,
    pub quote_increment: Decimal,
    pub min_base_trade_size: Option<Decimal>,
    pub min_quote_trade_size: Option<Decimal>,
}