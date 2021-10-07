use ligen::marshalling::MarshalFrom;
use crate::model::AskBid;
use rust_decimal::Decimal;
use std::str::FromStr;

#[repr(C)]
pub struct FFIAskBid {
    price: i64,
    qty: i64
}

impl MarshalFrom<FFIAskBid> for AskBid {
    fn marshal_from(from: FFIAskBid) -> Self {
        let qty = Decimal::from_str(&String::marshal_from(from.qty)).expect("Invalid number format.");
        let price = Decimal::from_str(&String::marshal_from(from.price)).expect("Invalid number format.");
        Self { qty, price }
    }
}