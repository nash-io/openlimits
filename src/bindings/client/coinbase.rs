use crate::bindings::string::FFIString;
use crate::bindings::environment::Environment;
use openlimits_coinbase::{CoinbaseParameters, CoinbaseCredentials};
use ligen::traits::marshalling::MarshalFrom;
use ligen_macro::inner_ligen;

inner_ligen! {
    ffi(CoinbaseParameters(name = "FFICoinbaseParameters")),
    marshal(
        FFICoinbaseParameters(
            name = "CoinbaseParameters"
        )
    ),
    csharp(
        ffi(
            FFICoinbaseParameters(
                name = "CoinbaseParameters"
            )
        ),
    )
}

#[repr(C, packed(1))]
pub struct FFICoinbaseParameters {
    environment: Environment,
    apiKey: *mut FFIString,
    apiSecret: *mut FFIString,
    passphrase: *mut FFIString
}

impl MarshalFrom<FFICoinbaseParameters> for CoinbaseParameters {
    fn marshal_from(from: FFICoinbaseParameters) -> Self {
        unsafe {
            let api_key = String::marshal_from(from.apiKey.read());
            let api_secret = String::marshal_from(from.apiSecret.read());
            let passphrase = String::marshal_from(from.passphrase.read());
            let credentials = if !api_key.is_empty() && !api_secret.is_empty() && !passphrase.is_empty() {
                Some(CoinbaseCredentials { api_key, api_secret, passphrase })
            } else {
                None
            };
            let environment = from.environment.into();
            Self { environment, credentials }
        }
    }
}