use serde::Deserialize;
use serde::Serialize;
use super::AccountUpdateBalance;

/// This struct represents the account update
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdate {
    #[serde(rename = "e")]
    pub event_type: String,
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "m")]
    pub maker_commision_rate: u64,
    #[serde(rename = "t")]
    pub taker_commision_rate: u64,
    #[serde(rename = "b")]
    pub buyer_commision_rate: u64,
    #[serde(rename = "s")]
    pub seller_commision_rate: u64,
    #[serde(rename = "T")]
    pub can_trade: bool,
    #[serde(rename = "W")]
    pub can_withdraw: bool,
    #[serde(rename = "D")]
    pub can_deposit: bool,
    #[serde(rename = "u")]
    pub last_account_update: u64,
    #[serde(rename = "B")]
    pub balance: Vec<AccountUpdateBalance>,
}