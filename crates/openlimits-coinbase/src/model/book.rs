use serde::Deserialize;
use serde::Serialize;

/// This struct represents the offer book
#[derive(Serialize, Deserialize, Debug)]
pub struct Book<T> {
    pub sequence: usize,
    pub bids: Vec<T>,
    pub asks: Vec<T>,
}