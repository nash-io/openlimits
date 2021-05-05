use chrono::Duration;
use serde::de::Visitor;
use serde::de;
use std::fmt;
use super::TimeInForce;

/// This struct uses the time in force enum
pub struct TimeInForceVisitor;

impl<'de> Visitor<'de> for TimeInForceVisitor {
    type Value = TimeInForce;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an string, either GTC, IOC, FOK, GTT,duration")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if v.starts_with("GTT,") {
            match v[4..].parse::<u64>() {
                Ok(v) => Ok(TimeInForce::GoodTillTime(Duration::milliseconds(v as i64))),
                _ => Err(E::custom(format!("Invalid GTG: {}", v))),
            }
        } else {
            match v {
                "GTC" => Ok(TimeInForce::GoodTillCancelled),
                "IOC" => Ok(TimeInForce::ImmediateOrCancelled),
                "FOK" => Ok(TimeInForce::FillOrKill),
                _ => Err(E::custom(format!("Invalid string: {}", v))),
            }
        }
    }
}