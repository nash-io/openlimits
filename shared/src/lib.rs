pub mod errors;

extern crate serde;
extern crate serde_json;

pub type Result<T> = std::result::Result<T, errors::OpenLimitError>;

pub mod string_to_float {
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
        enum StringToFloat {
            String(String),
        }

        let StringToFloat::String(s) = StringToFloat::deserialize(deserializer)?;
        s.parse().map_err(de::Error::custom)
    }
}

pub mod f64_nan_from_string {
    use std::fmt;

    use serde::{Deserialize, Deserializer, Serializer};

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
        enum F64NanFromString {
            String(String),
        }

        let F64NanFromString::String(s) = F64NanFromString::deserialize(deserializer)?;
        s.parse().or(Ok(std::f64::NAN))
    }
}

pub mod f64_opt_from_string {
    use std::fmt;

    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum F64OptFromString {
            String(Option<String>),
        }

        let F64OptFromString::String(s) = F64OptFromString::deserialize(deserializer)?;
        if let Some(s) = s {
            return s.parse().map(Some).or(Ok(None));
        }
        Ok(None)
    }
}
