use crate::Result;
use chrono::Utc;
use hex::encode as hexify;
use hmac::{Hmac, Mac, NewMac};
use reqwest::header;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_urlencoded;
use sha2::Sha256;
use url::Url;

use crate::errors::OpenLimitError;

type HmacSha256 = Hmac<Sha256>;

static BASE: &'static str = "https://www.binance.com";
static RECV_WINDOW: usize = 5000;

#[derive(Clone)]
pub struct Transport {
    credential: Option<(String, String)>,
    client: reqwest::Client,
    pub recv_window: usize,
}

impl Transport {
    pub fn new() -> Result<Self> {
        let default_headers = Transport::default_headers(None);
        let client = reqwest::Client::builder()
            .default_headers(default_headers)
            .build()?;

        Ok(Transport {
            credential: None,
            client: client,
            recv_window: RECV_WINDOW,
        })
    }

    pub fn with_credential(api_key: &str, api_secret: &str) -> Result<Self> {
        let default_headers = Transport::default_headers(Some(api_key));
        let client = reqwest::Client::builder()
            .default_headers(default_headers)
            .build()?;

        Ok(Transport {
            client: client,
            credential: Some((api_key.into(), api_secret.into())),
            recv_window: RECV_WINDOW,
        })
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
            headers.insert("X-MBX-APIKEY", header::HeaderValue::from_str(key).unwrap());
        };

        headers
    }

    pub async fn get<O, S>(&self, endpoint: &str, params: Option<S>) -> Result<O>
    where
        O: DeserializeOwned,
        S: Serialize,
    {
        let url = self.get_url(endpoint, false)?;
        Ok(self
            .client
            .get(url)
            .form(&params)
            .send()
            .await?
            .json::<O>()
            .await?)
    }

    pub async fn post<O, D>(&self, endpoint: &str, data: Option<D>) -> Result<O>
    where
        O: DeserializeOwned,
        D: Serialize,
    {
        let url = self.get_url(endpoint, false)?;
        Ok(self
            .client
            .post(url)
            .json(&data)
            .send()
            .await?
            .json::<O>()
            .await?)
    }

    pub async fn put<O, D>(&self, endpoint: &str, data: Option<D>) -> Result<O>
    where
        O: DeserializeOwned,
        D: Serialize,
    {
        let url = self.get_url(endpoint, false)?;
        Ok(self
            .client
            .put(url)
            .json(&data)
            .send()
            .await?
            .json::<O>()
            .await?)
    }

    pub async fn delete<O, Q>(&self, endpoint: &str, data: Option<Q>) -> Result<O>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        let url = self.get_url(endpoint, false)?;
        Ok(self
            .client
            .delete(url)
            .json(&data)
            .send()
            .await?
            .json::<O>()
            .await?)
    }

    pub async fn signed_get<O, Q>(&self, endpoint: &str, params: Option<Q>) -> Result<O>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        let mut url = self.get_url(endpoint, true)?;

        let (_, signature) = self.signature::<()>(&url, None)?;
        url.query_pairs_mut().append_pair("signature", &signature);

        Ok(self
            .client
            .get(url)
            .form(&params)
            .send()
            .await?
            .json::<O>()
            .await?)
    }

    pub async fn signed_post<D, O>(&self, endpoint: &str, data: Option<D>) -> Result<O>
    where
        O: DeserializeOwned,
        D: Serialize,
    {
        let mut url = self.get_url(endpoint, true)?;

        let (_, signature) = self.signature(&url, Some(&data))?;
        url.query_pairs_mut().append_pair("signature", &signature);

        Ok(self
            .client
            .post(url)
            .json(&data)
            .send()
            .await?
            .json::<O>()
            .await?)
    }

    pub async fn signed_put<O, Q>(&self, endpoint: &str, data: Option<Q>) -> Result<O>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        let mut url = self.get_url(endpoint, true)?;

        let (_, signature) = self.signature(&url, Some(&data))?;
        url.query_pairs_mut().append_pair("signature", &signature);

        Ok(self
            .client
            .put(url)
            .json(&data)
            .send()
            .await?
            .json::<O>()
            .await?)
    }

    pub async fn signed_delete<O, Q>(&self, endpoint: &str, data: Option<Q>) -> Result<O>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        let mut url = self.get_url(endpoint, true)?;

        let (_, signature) = self.signature(&url, Some(&data))?;
        url.query_pairs_mut().append_pair("signature", &signature);

        Ok(self
            .client
            .delete(url)
            .json(&data)
            .send()
            .await?
            .json::<O>()
            .await?)
    }

    pub fn get_url(&self, endpoint: &str, add_recv_window: bool) -> Result<Url> {
        let url = format!("{}{}", BASE, endpoint);
        let mut url = Url::parse(&url)?;

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
        let mut mac = HmacSha256::new_varkey(secret.as_bytes()).unwrap();
        let body = if let Some(_) = body {
            serde_urlencoded::to_string(body)?
        } else {
            String::from("")
        };

        let sign_message = match url.query() {
            Some(query) => format!("{}{}", query, body),
            None => format!("{}", body),
        };

        mac.update(sign_message.as_bytes());
        let signature = hexify(mac.finalize().into_bytes());
        Ok((key, signature))
    }
}
