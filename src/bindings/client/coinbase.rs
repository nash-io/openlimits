use crate::bindings::string::FFIString;
use crate::bindings::environment::Environment;
use crate::exchange::coinbase::{CoinbaseParameters, CoinbaseCredentials};
use ligen::marshalling::MarshalFrom;

#[repr(C, packed(1))]
pub struct FFICoinbaseParameters {
    environment: Environment,
    apiKey: FFIString,
    apiSecret: FFIString,
    passphrase: FFIString
}

impl MarshalFrom<FFICoinbaseParameters> for CoinbaseParameters {
    fn marshal_from(from: FFICoinbaseParameters) -> Self {
        let sandbox = match from.environment {
            Environment::Sandbox => true,
            _ => false
        };
        let api_key = String::marshal_from(from.apiKey);
        let api_secret = String::marshal_from(from.apiSecret);
        let passphrase = String::marshal_from(from.passphrase);
        let credentials = if !api_key.is_empty() && !api_secret.is_empty() && !passphrase.is_empty() {
            Some(CoinbaseCredentials { api_key, api_secret, passphrase })
        } else {
            None
        };
        Self { sandbox, credentials }
    }
}