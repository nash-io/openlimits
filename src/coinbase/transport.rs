use crate::errors::OpenLimitError;
use crate::Result;
use hex::encode as hexify;
use hmac::{Hmac, Mac, NewMac};
use reqwest;
use reqwest::header;
use reqwest::Method;
use reqwest::Response;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sha2::Sha256;
use std::time::SystemTime;
use url::Url;

type HmacSha256 = Hmac<Sha256>;

// static BASE_URI: &'static str = "https://api.coinbase.com";
static SANDBOX_URI: &'static str = "https://api-public.sandbox.pro.coinbase.com";
#[derive(Clone)]
pub struct Transport {
    secret_key: Option<String>,
    client: reqwest::Client,
}

impl Transport {
    pub fn new() -> Result<Self> {
        let default_headers = Transport::default_headers();

        let client = reqwest::Client::builder()
            .default_headers(default_headers)
            .build()?;

        Ok(Transport {
            client,
            secret_key: None,
        })
    }

    pub fn with_credential(api_key: &str, secret_key: &str, passphrase: &str) -> Result<Self> {
        let default_headers = Transport::default_headers_with_auth(&api_key, &passphrase);
        let client = reqwest::Client::builder()
            .default_headers(default_headers)
            .build()?;

        Ok(Transport {
            secret_key: Some(String::from(secret_key)),
            client,
        })
    }
    pub fn default_headers() -> header::HeaderMap<header::HeaderValue> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "USER-AGENT",
            header::HeaderValue::from_str("openlimit").unwrap(),
        );

        headers
    }

    pub fn default_headers_with_auth(
        api_key: &str,
        passphrase: &str,
    ) -> header::HeaderMap<header::HeaderValue> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "USER-AGENT",
            header::HeaderValue::from_str("openlimit").unwrap(),
        );

        headers.insert(
            "CB-ACCESS-KEY",
            header::HeaderValue::from_str(api_key).unwrap(),
        );

        headers.insert(
            "CB-ACCESS-PASSPHRASE",
            header::HeaderValue::from_str(passphrase).unwrap(),
        );

        headers
    }

    pub async fn get<O, S>(&self, endpoint: &str, params: Option<S>) -> Result<O>
    where
        O: DeserializeOwned,
        S: Serialize,
    {
        let url = self.get_url(endpoint, Some(&params))?;
        let request = self.client.get(url).send().await?;
        println!("{:?}", request.text().await?);

        let url = self.get_url(endpoint, Some(&params))?;
        let request = self.client.get(url).send().await?;

        Ok(self.response_handler(request).await?)
    }

    pub async fn get_signed<O, S>(&self, endpoint: &str, params: Option<S>) -> Result<O>
    where
        O: DeserializeOwned,
        S: Serialize,
    {
        let since_epoch_seconds = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Invalid SystemTime.")
            .as_secs();
        let url = self.get_url(endpoint, Some(&params))?;

        let signature = self.signature::<()>(&url, since_epoch_seconds, Method::GET, None)?;

        Ok(self
            .client
            .get(url)
            .header("CB-ACCESS-SIGN", signature)
            .header("CB-ACCESS-TIMESTAMP", since_epoch_seconds.to_string())
            .send()
            .await?
            .json::<O>()
            .await?)
    }

    pub fn get_url<Q>(&self, endpoint: &str, params: Option<&Q>) -> Result<Url>
    where
        Q: Serialize,
    {
        let url = format!("{}{}", SANDBOX_URI, endpoint);

        let mut url = Url::parse(&url)?;

        if params.is_some() {
            let query = serde_urlencoded::to_string(params)?;
            url.set_query(Some(&query));
        };

        Ok(url)
    }

    pub fn signature<D>(
        &self,
        url: &Url,
        timestamp: u64,
        method: Method,
        body: Option<&D>,
    ) -> Result<String>
    where
        D: Serialize,
    {
        let secret_key = match self.secret_key.as_ref() {
            None => Err(OpenLimitError::NoApiKeySet()),
            Some(v) => Ok(v),
        }?;

        let mut mac = HmacSha256::new_varkey(secret_key.as_bytes()).unwrap();

        let prefix: String = String::from(timestamp.to_string() + method.as_str());

        let body = if body.is_some() {
            serde_urlencoded::to_string(body)?
        } else {
            String::from("")
        };

        let sign_message = format!("{}{}{}", prefix, url.path(), body);

        mac.update(sign_message.as_bytes());
        let signature = hexify(mac.finalize().into_bytes());
        Ok(signature)
    }

    async fn response_handler<O>(&self, response: Response) -> Result<O>
    where
        O: DeserializeOwned,
    {
        match response.status() {
            StatusCode::OK => Ok(response.json::<O>().await?),
            StatusCode::INTERNAL_SERVER_ERROR => Err(OpenLimitError::InternalServerError()),
            StatusCode::SERVICE_UNAVAILABLE => Err(OpenLimitError::ServiceUnavailable()),
            StatusCode::UNAUTHORIZED => Err(OpenLimitError::Unauthorized()),
            // StatusCode::BAD_REQUEST => {
            //     let error: BinanceContentError = response.json().await?;

            //     Err(OpenLimitError::BinanceError(error).into())
            // }
            s => Err(OpenLimitError::UnkownResponse(format!(
                "Received response: {:?}",
                s
            ))),
        }
    }
}
