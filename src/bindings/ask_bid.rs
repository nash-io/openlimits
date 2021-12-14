use ligen::traits::marshalling::{MarshalFrom, MarshalInto};
use crate::model::AskBid;
use rust_decimal::Decimal;
use std::str::FromStr;
use crate::bindings::string::FFIString;

use ligen_macro::inner_ligen;

inner_ligen! {
    ffi(AskBid(name = "FFIAskBid")),
    csharp(ffi(FFIAskBid(name = "AskBid")))
}

#[repr(C)]
pub struct FFIAskBid {
    price: FFIString,
    qty: FFIString
}

impl MarshalFrom<FFIAskBid> for AskBid {
    fn marshal_from(from: FFIAskBid) -> Self {
        let qty = Decimal::from_str(&String::marshal_from(from.qty)).expect("Invalid number format.");
        let price = Decimal::from_str(&String::marshal_from(from.price)).expect("Invalid number format.");
        Self { qty, price }
    }
}

impl MarshalFrom<AskBid> for FFIAskBid {
    fn marshal_from(from: AskBid) -> Self {
        let qty = from.qty.marshal_into();
        let price = from.price.marshal_into();
        Self { price, qty }
    }
}