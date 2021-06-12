use rust_decimal::prelude::Decimal;

#[derive(Debug, Clone)]
pub struct KlineSummary {
    pub open_time: i64,
    pub open: Decimal,
    pub high: Decimal,
    pub low: Decimal,
    pub close: Decimal,
    pub volume: Decimal,
    pub close_time: i64,
    pub quote_asset_volume: Decimal,
    pub number_of_trades: i64,
    pub taker_buy_base_asset_volume: Decimal,
    pub taker_buy_quote_asset_volume: Decimal,
}