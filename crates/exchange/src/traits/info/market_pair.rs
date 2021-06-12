use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct MarketPair {
    pub base: String,
    pub quote: String,
    pub symbol: String,
    pub base_increment: Decimal,
    pub quote_increment: Decimal,
    pub min_base_trade_size: Option<Decimal>,
    pub min_quote_trade_size: Option<Decimal>,
}