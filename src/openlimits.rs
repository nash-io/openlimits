#[cfg(feature = "bindings")]
use ligen_macro::inner_ligen;
#[cfg(feature = "bindings")]
inner_ligen!(ignore);

use crate::prelude::*;
use crate::exchange::shared::Result;

/// Can be used to initiate exchanges
pub struct OpenLimits {}

impl OpenLimits {
    pub async fn instantiate<E: Exchange>(parameters: E::InitParams) -> Result<E> {
        Ok(E::new(parameters).await?)
    }
}
