#![deny(unstable_features)]
#![allow(clippy::too_many_arguments)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unsafe_code)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]

pub mod exchange;
pub mod reconnectable_ws;
pub mod model;
pub mod prelude;
pub mod exchange_ws;
pub mod any_exchange;
mod errors;
mod exchange_traits;
mod exchange_info;
pub(crate) mod shared;

use crate::prelude::*;
pub use crate::shared::Result;

pub struct OpenLimits {}

impl OpenLimits {
    pub async fn instantiate<E: Exchange>(parameters: E::InitParams) -> Result<E> {
        Ok(E::new(parameters).await?)
    }
}

