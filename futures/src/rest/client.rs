use super::models::Request;
use crate::errors::KrakenError;
use base64::{decode as b64decode, encode as b64encode};
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use failure::Fail;
use fehler::{throw, throws};
use http::Method;
use log::error;
use reqwest::{Client, Response};
use ring::digest::{digest, SHA256};
use ring::hmac;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string as to_jstring, to_value};
use serde_urlencoded::to_string as to_ustring;
use std::str;
use url::Url;

const REST_URL: &'static str = "https://futures.kraken.com/derivatives/api/v3";

#[derive(Clone, Builder)]
pub struct KrakenRest {
    client: Client,
    #[builder(default)]
    credential: Option<(String, String)>,
}

impl Default for KrakenRest {
    fn default() -> Self {
        Self::new()
    }
}

impl KrakenRest {
    pub fn new() -> Self {
        KrakenRest {
            client: Client::new(),
            credential: None,
        }
    }

    pub fn with_credential(api_key: &str, api_secret: &str) -> Self {
        KrakenRest {
            client: Client::new(),
            credential: Some((api_key.into(), api_secret.into())),
        }
    }

    pub fn builder() -> KrakenRestBuilder {
        KrakenRestBuilder::default()
    }

    #[throws(failure::Error)]
    pub async fn request<R>(&self, req: R) -> R::Response
    where
        R: Request,
        R::Response: DeserializeOwned,
    {
        let url = format!("{}{}", &*REST_URL, R::ENDPOINT);
        let url = Url::parse(&url)?;

        let nonce = Utc::now().timestamp_millis();

        let mut query = vec![];

        if R::HAS_PAYLOAD {
            query = req.to_url_query();
            query.push(("nonce".to_string(), nonce.to_string()));
        }

        let mut req = match R::METHOD {
            Method::GET => self.client.request(R::METHOD, url.clone()).query(&query),
            Method::POST => self.client.request(R::METHOD, url.clone()).form(&query),
            _ => unreachable!("Unsupported HTTP method"),
        };

        if R::SIGNED {
            let (key, signature) = self.signature(&url, &to_ustring(&query)?, nonce)?;
            req = req.header("APIKey", key).header("Authent", signature).header("Nonce", nonce);
        }

        let resp = req
            .header("User-Agent", "kraken-rs")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .send()
            .await?;

        self.handle_response(resp).await?
    }

    #[throws(failure::Error)]
    fn check_key(&self) -> (&str, &str) {
        match self.credential.as_ref() {
            None => throw!(KrakenError::NoApiKeySet),
            Some((k, s)) => (k.as_str(), s.as_str()),
        }
    }

    #[throws(failure::Error)]
    pub(crate) fn signature(&self, url: &Url, body: &str, nonce: i64) -> (&str, String) {
        let (key, secret) = self.check_key()?;

        // Concatenate postData + nonce + endpointPath
        // Hash the result of step 1 with the SHA-256 algorithm
        // Base64-decode your api_secret
        // Use the result of step 3 to hash the result of the step 2 with the HMAC-SHA-512 algorithm
        // Base64-encode the result of step 4

        let mut message = body.as_bytes().to_vec();
        message.extend(nonce.to_string().bytes());
        message.extend(url.path().trim_start_matches("/derivatives").bytes());
        let digest = digest(&SHA256, &message);

        let signed_key = hmac::Key::new(hmac::HMAC_SHA512, &b64decode(secret)?);
        let signature = hmac::sign(&signed_key, digest.as_ref());
        let signature = b64encode(&signature);

        (key, signature)
    }

    #[throws(failure::Error)]
    async fn handle_response<T: DeserializeOwned>(&self, resp: Response) -> T {
        let status = resp.status();
        let resp = resp.text().await?;

        if let Ok(p) = from_str::<KrakenRestResponse<T>>(&resp) {
            return p.payload;
        } else if let Ok(e) = from_str::<KrakenRestErrorResponse>(&resp) {
            throw!(KrakenError::from(e))
        } else {
            throw!(KrakenError::CannotDeserializeResponse(resp))
        }
    }
}

trait ToUrlQuery: Serialize {
    fn to_url_query_string(&self) -> String {
        let vec = self.to_url_query();
        vec.into_iter().map(|(k, v)| format!("{}={}", k, v)).collect::<Vec<_>>().join("&")
    }

    fn to_url_query(&self) -> Vec<(String, String)> {
        let v = to_value(self).unwrap();
        let v = v.as_object().unwrap();
        let mut vec = vec![];

        for (key, value) in v.into_iter() {
            if value.is_null() {
                continue;
            } else if value.is_string() {
                vec.push((key.clone(), value.as_str().unwrap().to_string()))
            } else {
                vec.push((key.clone(), to_jstring(value).unwrap()))
            }
        }

        vec.sort_by(|a, b| a.0.cmp(&b.0));

        vec
    }
}

impl<S: Serialize> ToUrlQuery for S {}

fn show(bs: &[u8]) -> String {
    use std::ascii::escape_default;
    let mut visible = String::new();
    for &b in bs {
        let part: Vec<u8> = escape_default(b).collect();
        visible.push_str(str::from_utf8(&part).unwrap());
    }
    visible
}

impl From<KrakenRestErrorResponse> for KrakenError {
    fn from(error: KrakenRestErrorResponse) -> KrakenError {
        KrakenError::KrakenError(error.error)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct KrakenRestResponse<T> {
    result: Success,
    #[serde(rename = "serverTime")]
    pub(crate) server_time: DateTime<Utc>,
    #[serde(flatten)]
    pub(crate) payload: T,
}

// The error response from bitmex;
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct KrakenRestErrorResponse {
    result: Error,
    pub(crate) error: String,
}

#[derive(Deserialize, Debug, Clone)]
enum Error {
    #[serde(rename = "error")]
    Error,
}

#[derive(Deserialize, Debug, Clone)]
enum Success {
    #[serde(rename = "success")]
    Success,
}
