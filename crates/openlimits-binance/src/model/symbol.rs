use serde::Deserialize;
use serde::Serialize;
use super::SymbolFilter;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Symbol {
    pub symbol: String,
    pub status: String,
    pub base_asset: String,
    pub base_asset_precision: u32,
    pub quote_asset: String,
    pub quote_precision: u32,
    pub order_types: Vec<String>,
    pub iceberg_allowed: bool,
    pub filters: Vec<SymbolFilter>,
}