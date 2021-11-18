use serde::Deserialize;
use serde::Serialize;

/// This struct contains the auth fields
#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
    pub signature: String,
    pub key: String,
    pub passphrase: String,
    pub timestamp: String,
}