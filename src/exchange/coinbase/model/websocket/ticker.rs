use rust_decimal::prelude::Decimal;
use serde::Deserialize;
use super::OrderSide;
use super::shared::string_to_decimal;
use super::shared::string_to_opt_decimal;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
#[serde(rename_all = "camelCase")]
pub enum Ticker {
    Full {
        trade_id: usize,
        sequence: usize,
        time: String,
        product_id: String,
        #[serde(with = "string_to_decimal")]
        price: Decimal,
        side: OrderSide,
        #[serde(with = "string_to_decimal")]
        last_size: Decimal,
        #[serde(with = "string_to_opt_decimal")]
        best_bid: Option<Decimal>,
        #[serde(with = "string_to_opt_decimal")]
        best_ask: Option<Decimal>,
    },
    Empty {
        sequence: usize,
        product_id: String,
        #[serde(with = "string_to_opt_decimal")]
        price: Option<Decimal>,
    },
}

impl Ticker {
    pub fn price(&self) -> Decimal {
        match self {
            Ticker::Full { price, .. } => *price,
            Ticker::Empty { price, .. } => price.expect("Couldn't get price."),
        }
    }

    pub fn time(&self) -> Option<&String> {
        match self {
            Ticker::Full { time, .. } => Some(time),
            Ticker::Empty { .. } => None,
        }
    }

    pub fn sequence(&self) -> &usize {
        match self {
            Ticker::Full { sequence, .. } => sequence,
            Ticker::Empty { sequence, .. } => sequence,
        }
    }

    pub fn bid(&self) -> Option<Decimal> {
        match self {
            Ticker::Full { best_bid, .. } => Some(best_bid.expect("Couldn't get best bid.")),
            Ticker::Empty { .. } => None,
        }
    }

    pub fn ask(&self) -> Option<Decimal> {
        match self {
            Ticker::Full { best_ask, .. } => Some(best_ask.expect("Couldn't get best ask.")),
            Ticker::Empty { .. } => None,
        }
    }
}