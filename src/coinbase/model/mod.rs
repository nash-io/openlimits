use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: String,
    pub display_name: String,
    pub quote_currency: String,
    pub base_currency: String,
    #[serde(with = "string_or_float")]
    pub base_increment: f64,
    #[serde(with = "string_or_float")]
    pub quote_increment: f64,
    #[serde(with = "string_or_float")]
    pub base_min_size: f64,
    #[serde(with = "string_or_float")]
    pub base_max_size: f64,
    pub min_market_funds: String,
    pub max_market_funds: String,
    pub status: String,
    pub status_message: String,
    pub cancel_only: bool,
    pub limit_only: bool,
    pub post_only: bool,
    pub trading_disabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trade {
    pub trade_id: i64,
    pub time: String,
    pub size: String,
    #[serde(with = "string_or_float")]
    pub price: f64,
    pub side: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Book<T> {
    pub sequence: usize,
    pub bids: Vec<T>,
    pub asks: Vec<T>,
}

pub trait BookLevel {
    fn level() -> u8;
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(from = "(f64, f64, usize)")]
pub struct BookRecordL1 {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub size: f64,
    pub num_orders: usize,
}

impl BookLevel for BookRecordL1 {
    fn level() -> u8 {
        1
    }
}

impl From<(f64, f64, usize)> for BookRecordL1 {
    fn from(t: (f64, f64, usize)) -> Self {
        Self {
            price: t.0,
            size: t.1,
            num_orders: t.2,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(from = "(f64, f64, usize)")]
pub struct BookRecordL2 {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub size: f64,
    pub num_orders: usize,
}

impl BookLevel for BookRecordL2 {
    fn level() -> u8 {
        2
    }
}

impl From<(f64, f64, usize)> for BookRecordL2 {
    fn from(t: (f64, f64, usize)) -> Self {
        Self {
            price: t.0,
            size: t.1,
            num_orders: t.2,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(from = "(f64, f64, String)")]
pub struct BookRecordL3 {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub size: f64,
    pub order_id: String,
}

impl BookLevel for BookRecordL3 {
    fn level() -> u8 {
        3
    }
}

impl From<(f64, f64, String)> for BookRecordL3 {
    fn from(t: (f64, f64, String)) -> Self {
        Self {
            price: t.0,
            size: t.1,
            order_id: t.2,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bids {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
    pub num_orders: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asks {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
    pub num_orders: i64,
}

mod string_or_float {
    use std::fmt;

    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => s.parse().map_err(de::Error::custom),
            StringOrFloat::Float(i) => Ok(i),
        }
    }
}
