use chrono::Utc;
use hex::encode as hexify;
use hmac::{Hmac, Mac, NewMac};
use reqwest::header;
use reqwest::Response;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sha2::Sha512;
use url::Url;

use crate::{
    errors::{ExmoContentError, OpenLimitsError},
    shared::Result,
};

type HmacSha512 = Hmac<Sha512>;

#[derive(Clone)]
pub struct Transport {
    credential: Option<(String, String)>,
    client: reqwest::Client,
    base_url: String,
}

impl Transport {
    pub fn new(sandbox: bool) -> Result<Self> {
        let default_headers = Transport::default_headers(None, None);

        let client = reqwest::Client::builder()
            .default_headers(default_headers)
            .build()?;

        Ok(Transport {
            credential: None,
            client,
            base_url: Transport::get_base_url(sandbox),
        })
    }

    pub fn with_credential(api_key: &str, api_secret: &str, sandbox: bool) -> Result<Self> {
        let default_headers = Transport::default_headers(Some(api_key), Some(api_secret));

        let client = reqwest::Client::builder()
            .default_headers(default_headers)
            .build()?;

        Ok(Transport {
            client,
            credential: Some((api_key.into(), api_secret.into())),
            base_url: Transport::get_base_url(sandbox),
        })
    }

    // TODO: check if the sandbox endpoint exists at exmo?
    fn get_base_url(sandbox: bool) -> String {
        if sandbox {
            String::from("https://api.exmo.com")
        } else {
            String::from("https://api.exmo.com")
        }
    }

    pub fn default_headers(
        api_key: Option<&str>,
        api_secret: Option<&str>,
    ) -> header::HeaderMap<header::HeaderValue> {
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
                "Key",
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
        let url = self.get_url(endpoint, params)?;
        let request = self.client.get(url).send().await?;

        Ok(self.response_handler(request).await?)
    }

    pub async fn post<O, D>(&self, endpoint: &str, data: Option<&D>) -> Result<O>
    where
        O: DeserializeOwned,
        D: Serialize,
    {
        let url = self.get_url::<()>(endpoint, None)?;
        let request = self.client.post(url).form(&data).send().await?;

        Ok(self.response_handler(request).await?)
    }

    pub async fn put<O, D>(&self, endpoint: &str, data: Option<D>) -> Result<O>
    where
        O: DeserializeOwned,
        D: Serialize,
    {
        let url = self.get_url::<()>(endpoint, None)?;
        let request = self.client.put(url).form(&data).send().await?;

        Ok(self.response_handler(request).await?)
    }

    pub async fn delete<O, Q>(&self, endpoint: &str, data: Option<&Q>) -> Result<O>
    where
        O: DeserializeOwned,
        Q: Serialize,
    {
        let url = self.get_url::<()>(endpoint, None)?;
        let request = self.client.delete(url).form(&data).send().await?;

        Ok(self.response_handler(request).await?)
    }

    pub async fn signed_get<O, S>(&self, endpoint: &str, params: Option<&S>) -> Result<O>
    where
        O: DeserializeOwned,
        S: Serialize,
    {
        let mut url = self.get_url(endpoint, params)?;

        let (_, signature) = self.signature::<()>(&url, None)?;
        url.query_pairs_mut().append_pair("signature", &signature);

        let request = self.client.get(url).send().await?;

        Ok(self.response_handler(request).await?)
    }

    pub fn get_url<Q>(&self, endpoint: &str, params: Option<&Q>) -> Result<Url>
    where
        Q: Serialize,
    {
        let url = format!("{}{}", self.base_url, endpoint);

        let mut url = Url::parse(&url)?;

        if params.is_some() {
            let query = serde_urlencoded::to_string(params)?;
            url.set_query(Some(&query));
        };

        Ok(url)
    }

    fn check_key(&self) -> Result<(&str, &str)> {
        match self.credential.as_ref() {
            None => Err(OpenLimitsError::NoApiKeySet()),
            Some((k, s)) => Ok((k, s)),
        }
    }

    pub fn signature<D>(&self, url: &Url, body: Option<&D>) -> Result<(&str, String)>
    where
        D: Serialize,
    {
        let (key, secret) = self.check_key()?;
        // TODO: add nonce!!!
        let mut mac =
            HmacSha512::new_varkey(secret.as_bytes()).expect("Couldn't construct hmac from bytes.");
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
            StatusCode::INTERNAL_SERVER_ERROR => Err(OpenLimitsError::InternalServerError()),
            StatusCode::SERVICE_UNAVAILABLE => Err(OpenLimitsError::ServiceUnavailable()),
            StatusCode::UNAUTHORIZED => Err(OpenLimitsError::Unauthorized()),
            StatusCode::BAD_REQUEST => {
                let error: ExmoContentError = response.json().await?;

                Err(OpenLimitsError::ExmoError(error))
            }
            s => Err(OpenLimitsError::UnkownResponse(format!(
                "Received response: {:?}",
                s
            ))),
        }
    }
}
