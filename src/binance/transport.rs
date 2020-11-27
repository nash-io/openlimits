use chrono::Utc;
use hex::encode as hexify;
use hmac::{Hmac, Mac, NewMac};
use reqwest::header;
use reqwest::Response;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sha2::Sha256;
use url::Url;

use crate::{
    errors::{BinanceContentError, OpenLimitError},
    shared::Result,
};

type HmacSha256 = Hmac<Sha256>;

static RECV_WINDOW: usize = 7000;

#[derive(Clone)]
pub struct Transport {
    credential: Option<(String, String)>,
    client: reqwest::Client,
    pub recv_window: usize,
    base_url: String,
}

impl Transport {
    pub fn new(sandbox: bool) -> Result<Self> {
        let default_headers = Transport::default_headers(None);
        let client = reqwest::Client::builder()
            .default_headers(default_headers)
            .build()?;

        Ok(Transport {
            credential: None,
            client,
            recv_window: RECV_WINDOW,
            base_url: Transport::get_base_url(sandbox),
        })
    }

    pub fn with_credential(api_key: &str, api_secret: &str, sandbox: bool) -> Result<Self> {
        let default_headers = Transport::default_headers(Some(api_key));
        let client = reqwest::Client::builder()
            .default_headers(default_headers)
            .build()?;

        Ok(Transport {
            client,
            credential: Some((api_key.into(), api_secret.into())),
            recv_window: RECV_WINDOW,
            base_url: Transport::get_base_url(sandbox),
        })
    }

    fn get_base_url(sandbox: bool) -> String {
        if sandbox {
            String::from("https://testnet.binance.vision")
        } else {
            String::from("https://api.binance.com")
        }
    }

    pub fn default_headers(api_key: Option<&str>) -> header::HeaderMap<header::HeaderValue> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("open_limit"),
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        if let Some(key) = api_key {
            headers.insert(
                "X-MBX-APIKEY",
                header::HeaderValue::from_str(key).expect("Couldn't parse API key from string."),
            );
        };

        headers
    }

    pub async fn get<O, S>(&self, endpoint: &str, params: Option<&S>) -> Result<O>
    where
        O: DeserializeOwned,
        S: Serialize,
    {
        let url = self.get_url(endpoint, params, false)?;
        let request = self.client.get(url).send().await?;

        Ok(self.response_handler(request).await?)
    }

    pub async fn post<O, D>(&self, endpoint: &str, data: Option<&D>) -> Result<O>
    where
        O: DeserializeOwned,
        D: Serialize,
    {
        let url = self.get_url::<()>(endpoint, None, false)?;
        let request = self.client.post(url).form(&data).send().await?;

        Ok(self.response_handler(request).await?)
    }

    pub async fn put<O, D>(&self, endpoint: &str, data: Option<D>) -> Result<O>
    where
        O: DeserializeOwned,
        D: Serialize,
    {
        let url = self.get_url::<()>(endpoint, None, false)?;
        let request = self.client.put(url).form(&data).send().await?;

        Ok(self.response_handler(request).await?)
    }

    pub async fn delete<O, Q>(&self, endpoint: &str, data: Option<&Q>) -> Result<O>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        let url = self.get_url::<()>(endpoint, None, false)?;
        let request = self.client.delete(url).form(&data).send().await?;

        Ok(self.response_handler(request).await?)
    }

    pub async fn signed_get<O, S>(&self, endpoint: &str, params: Option<&S>) -> Result<O>
    where
        O: DeserializeOwned,
        S: Serialize,
    {
        let mut url = self.get_url(endpoint, params, true)?;

        let (_, signature) = self.signature::<()>(&url, None)?;
        url.query_pairs_mut().append_pair("signature", &signature);

        let request = self.client.get(url).send().await?;

        Ok(self.response_handler(request).await?)
    }

    pub async fn signed_post<D, O>(&self, endpoint: &str, data: Option<&D>) -> Result<O>
    where
        O: DeserializeOwned,
        D: Serialize,
    {
        let mut url = self.get_url::<()>(endpoint, None, true)?;

        let (_, signature) = self.signature(&url, data)?;
        url.query_pairs_mut().append_pair("signature", &signature);

        let request = self.client.post(url).form(&data).send().await?;
        Ok(self.response_handler(request).await?)
    }

    pub async fn signed_put<O, Q>(&self, endpoint: &str, data: Option<&Q>) -> Result<O>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        let mut url = self.get_url::<()>(endpoint, None, true)?;

        let (_, signature) = self.signature(&url, data)?;
        url.query_pairs_mut().append_pair("signature", &signature);

        let request = self.client.put(url).form(&data).send().await?;

        Ok(self.response_handler(request).await?)
    }

    pub async fn signed_delete<O, Q>(&self, endpoint: &str, data: Option<&Q>) -> Result<O>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        let mut url = self.get_url::<()>(endpoint, None, true)?;

        let (_, signature) = self.signature(&url, data)?;
        url.query_pairs_mut().append_pair("signature", &signature);

        let request = self.client.delete(url).form(&data).send().await?;

        Ok(self.response_handler(request).await?)
    }

    pub fn get_url<Q>(
        &self,
        endpoint: &str,
        params: Option<&Q>,
        add_recv_window: bool,
    ) -> Result<Url>
    where
        Q: Serialize,
    {
        let url = format!("{}{}", self.base_url, endpoint);

        let mut url = Url::parse(&url)?;

        if params.is_some() {
            let query = serde_urlencoded::to_string(params)?;
            url.set_query(Some(&query));
        };

        if add_recv_window {
            url.query_pairs_mut()
                .append_pair("timestamp", &Utc::now().timestamp_millis().to_string());
            url.query_pairs_mut()
                .append_pair("recvWindow", &self.recv_window.to_string());
        };
        Ok(url)
    }

    fn check_key(&self) -> Result<(&str, &str)> {
        match self.credential.as_ref() {
            None => Err(OpenLimitError::NoApiKeySet()),
            Some((k, s)) => Ok((k, s)),
        }
    }

    pub fn signature<D>(&self, url: &Url, body: Option<&D>) -> Result<(&str, String)>
    where
        D: Serialize,
    {
        let (key, secret) = self.check_key()?;
        let mut mac =
            HmacSha256::new_varkey(secret.as_bytes()).expect("Couldn't construct hmac from bytes.");
        let body = if body.is_some() {
            serde_urlencoded::to_string(body)?
        } else {
            String::from("")
        };

        let sign_message = match url.query() {
            Some(query) => format!("{}{}", query, body),
            None => body,
        };

        mac.update(sign_message.as_bytes());
        let signature = hexify(mac.finalize().into_bytes());
        Ok((key, signature))
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
            StatusCode::BAD_REQUEST => {
                let error: BinanceContentError = response.json().await?;

                Err(OpenLimitError::BinanceError(error))
            }
            s => Err(OpenLimitError::UnkownResponse(format!(
                "Received response: {:?}",
                s
            ))),
        }
    }
}
