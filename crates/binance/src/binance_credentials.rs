/// This struct represents the credentials and receives the api key and api secret as parameters.
#[derive(Clone, Debug)]
pub struct BinanceCredentials {
    pub api_key: String,
    pub api_secret: String,
}