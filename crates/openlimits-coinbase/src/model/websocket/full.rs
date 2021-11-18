use rust_decimal::prelude::Decimal;
use serde::Deserialize;
use super::Activate;
use super::Change;
use super::Done;
use super::Open;
use super::Match;
use super::Received;


#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum Full {
    Received(Received),
    Open(Open),
    Done(Done),
    Match(Match),
    Change(Change),
    Activate(Activate),
}

impl Full {
    pub fn price(&self) -> Option<&Decimal> {
        match self {
            Full::Received(Received::Limit { price, .. }) => Some(price),
            Full::Received(Received::Market { .. }) => None,
            Full::Open(Open { price, .. }) => Some(price),
            Full::Done(Done::Limit { price, .. }) => Some(price),
            Full::Done(Done::Market { .. }) => None,
            Full::Match(Match { price, .. }) => Some(price),
            Full::Change(Change { price, .. }) => price.as_ref(),
            Full::Activate(Activate { .. }) => None,
        }
    }

    pub fn time(&self) -> Option<&String> {
        match self {
            Full::Received(Received::Limit { time, .. }) => Some(time),
            Full::Received(Received::Market { time, .. }) => Some(time),
            Full::Open(Open { time, .. }) => Some(time),
            Full::Done(Done::Limit { time, .. }) => Some(time),
            Full::Done(Done::Market { time, .. }) => Some(time),
            Full::Match(Match { time, .. }) => Some(time),
            Full::Change(Change { time, .. }) => Some(time),
            Full::Activate(Activate { .. }) => None,
        }
    }

    pub fn sequence(&self) -> Option<&usize> {
        match self {
            Full::Received(Received::Limit { sequence, .. }) => Some(sequence),
            Full::Received(Received::Market { sequence, .. }) => Some(sequence),
            Full::Open(Open { sequence, .. }) => Some(sequence),
            Full::Done(Done::Limit { sequence, .. }) => sequence.as_ref(),
            Full::Done(Done::Market { sequence, .. }) => Some(sequence),
            Full::Match(Match { sequence, .. }) => Some(sequence),
            Full::Change(Change { sequence, .. }) => Some(sequence),
            Full::Activate(Activate { .. }) => None,
        }
    }
}