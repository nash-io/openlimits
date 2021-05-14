/// This struct represents the coinbase credentials
#[derive(Clone)]
pub struct CoinbaseCredentials {
    pub api_key: String,
    pub api_secret: String,
    pub passphrase: String,
}