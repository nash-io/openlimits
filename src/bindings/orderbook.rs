// use crate::model::OrderBookResponse;
// use crate::bindings::client::FFIAskBid;
// use crate::bindings::vector::FFIVector;
//
// #[repr(C)]
// pub struct FFIOrderBookResponse {
//     update_id: *mut u64,
//     last_update_id: *mut u64,
//     bids: FFIVector<FFIAskBid>,
//     asks: FFIVector<FFIAskBid>
// }
//
// impl MarshalFrom<OrderBookResponse> for FFIOrderBookResponse {
//     fn marshal_from(value: OrderBookResponse) -> Self {
//
//         Self { update_id, last_update_id, bids, asks }
//     }
// }