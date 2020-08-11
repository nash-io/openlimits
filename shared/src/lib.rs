pub mod errors;

extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate chrono;

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

pub mod datetime_from_string {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::fmt;

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<chrono::DateTime<chrono::Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum DatetimeFromString {
            String(String),
        }

        let DatetimeFromString::String(s) = DatetimeFromString::deserialize(deserializer)?;
        match chrono::DateTime::parse_from_rfc3339(&s) {
            Ok(d) => Ok(d.with_timezone(&chrono::Utc)),
            Err(err) => Err(serde::de::Error::custom(err)),
        }
    }
}
