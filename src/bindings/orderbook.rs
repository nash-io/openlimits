use crate::model::OrderBookResponse;
use crate::bindings::vector::FFIVector;
use ligen_macro::inner_ligen;
use std::ptr::null_mut;
use ligen::marshalling::MarshalFrom;
use crate::bindings::ask_bid::FFIAskBid;

# Replicate generics logic to struct fields.
# We should have a FFI and a normal structure.

inner_ligen! {
    ffi(OrderBookResponse(name = "FFIOrderBookResponse")),
    csharp()
}

#[repr(C)]
pub struct FFIOrderBookResponse {
    update_id: *mut u64,
    last_update_id: *mut u64,
    bids: FFIVector<FFIAskBid>,
    asks: FFIVector<FFIAskBid>
}

impl MarshalFrom<OrderBookResponse> for FFIOrderBookResponse {
    fn marshal_from(_value: OrderBookResponse) -> Self {
        let update_id = null_mut();
        let last_update_id = null_mut();
        let bids = Default::default();
        let asks = Default::default();
        Self { update_id, last_update_id, bids, asks }
    }
}
