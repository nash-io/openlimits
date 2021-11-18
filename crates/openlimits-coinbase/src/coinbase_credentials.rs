/// This struct represents the coinbase credentials
#[derive(Clone, Debug)]
pub struct CoinbaseCredentials {
    pub api_key: String,
    pub api_secret: String,
    pub passphrase: String,
}