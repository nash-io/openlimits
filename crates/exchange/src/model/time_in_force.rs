use chrono::Duration;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use super::TimeInForceVisitor;

/// This enum represents time in force
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TimeInForce {
    GoodTillCancelled,
    ImmediateOrCancelled,
    FillOrKill,
    // Representing 'good till time' as a duration works for both Nash and Coinbase
    GoodTillTime(Duration),
}

impl<'de> Deserialize<'de> for TimeInForce {
    fn deserialize<D>(deserializer: D) -> Result<TimeInForce, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(TimeInForceVisitor)
    }
}

impl Serialize for TimeInForce {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match *self {
            TimeInForce::GoodTillCancelled => String::from("GTC"),
            TimeInForce::ImmediateOrCancelled => String::from("IOC"),
            TimeInForce::FillOrKill => String::from("FOK"),
            TimeInForce::GoodTillTime(d) => format!("GTT,{}", d.num_milliseconds()),
        };
        serializer.serialize_str(s.as_str())
    }
}

impl Default for TimeInForce {
    fn default() -> Self {
        TimeInForce::GoodTillCancelled
    }
}