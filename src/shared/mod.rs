pub mod errors;
pub mod exchange_info;

extern crate chrono;
extern crate serde;
extern crate serde_json;

pub type Result<T> = std::result::Result<T, errors::OpenLimitError>;

pub mod string_to_decimal {
    use std::fmt;

    use rust_decimal::prelude::*;
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringToDecimal {
            String(String),
        }

        let StringToDecimal::String(s) = StringToDecimal::deserialize(deserializer)?;
        Decimal::from_str(&s).map_err(de::Error::custom)
    }
}

pub mod string_to_opt_decimal {
    use std::fmt;

    use rust_decimal::prelude::*;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringToOptDecimal {
            String(Option<String>),
        }

        let StringToOptDecimal::String(s) = StringToOptDecimal::deserialize(deserializer)?;
        if let Some(s) = s {
            return Decimal::from_str(&s).map(Some).or(Ok(None));
        }
        Ok(None)
    }
}

pub mod naive_datetime_from_string {
    use chrono::naive::NaiveDateTime;
    use serde::{Deserialize, Deserializer, Serializer};
    use std::fmt;

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum DatetimeFromString {
            String(String),
        }

        let DatetimeFromString::String(s) = DatetimeFromString::deserialize(deserializer)?;

        NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S.%fZ")
            .map_err(|res| serde::de::Error::custom(res))
    }
}

pub mod opt_naive_datetime_from_string {
    use chrono::naive::NaiveDateTime;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(value) = value {
            return serializer.collect_str(&value);
        }
        serializer.serialize_none()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum OptDatetimeFromString {
            String(Option<String>),
        }

        let OptDatetimeFromString::String(s) = OptDatetimeFromString::deserialize(deserializer)?;
        if let Some(s) = s {
            return NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S.%fZ")
                .map(Some)
                .or(Ok(None));
        }
        Ok(None)
    }
}

pub fn timestamp_to_datetime(timestamp: u64) -> chrono::naive::NaiveDateTime {
    let seconds = (timestamp / 1000) as i64;
    let nanos = ((timestamp % 1000) * 1_000_000) as u32;

    chrono::NaiveDateTime::from_timestamp(seconds, nanos)
}
