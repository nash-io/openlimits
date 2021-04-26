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

use crate::prelude::*;
pub use crate::shared::Result;

pub mod exchange;
pub mod model;
pub mod prelude;
pub mod errors;
pub(crate) mod shared;

pub struct OpenLimits {}

impl OpenLimits {
    pub async fn instantiate<E: Exchange>(parameters: E::InitParams) -> Result<E> {
        Ok(E::new(parameters).await?)
    }
}

