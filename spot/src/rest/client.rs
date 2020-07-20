use super::models::Request;
use crate::errors::{KrakenError, KrakenErrorResponse, KrakenResponse};
use base64::{decode as b64decode, encode as b64encode};
use chrono::Utc;
use derive_builder::Builder;
use fehler::{throw, throws};
use http::Method;
use log::error;
use reqwest::{Client, Response};
use ring::digest::{digest, SHA256};
use ring::hmac;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_str, to_string as to_jstring, to_value};
use serde_urlencoded::to_string as to_ustring;
use std::ascii::escape_default;
use std::str;
use url::Url;

fn show(bs: &[u8]) -> String {
    let mut visible = String::new();
    for &b in bs {
        let part: Vec<u8> = escape_default(b).collect();
        visible.push_str(str::from_utf8(&part).unwrap());
    }
    visible
}

const REST_URL: &'static str = "https://api.kraken.com";

#[derive(Clone, Builder)]
pub struct Kraken {
    client: Client,
    #[builder(default)]
    credential: Option<(String, String)>,
}

impl Default for Kraken {
    fn default() -> Self {
        Self::new()
    }
}

impl Kraken {
    pub fn new() -> Self {
        Kraken {
            client: Client::new(),
            credential: None,
        }
    }

    pub fn with_credential(api_key: &str, api_secret: &str) -> Self {
        Kraken {
            client: Client::new(),
            credential: Some((api_key.into(), api_secret.into())),
        }
    }

    pub fn builder() -> KrakenBuilder {
        KrakenBuilder::default()
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

        let mut body = "".to_string();

        if R::HAS_PAYLOAD {
            let mut uqs = req.to_url_query();

            uqs.push(("nonce".to_string(), nonce.to_string()));
            body = to_ustring(&uqs)?;
        }

        let mut builder = self.client.request(Method::POST, url.clone());

        if R::SIGNED {
            let (key, signature) = self.signature(&url, &body, nonce)?;
            builder = builder.header("API-Key", key).header("API-Sign", signature);
        }

        let resp = builder
            .header("User-Agent", "kraken-rs")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
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
        println!("input to signature: {}, {}", url.path(), body);
        let (key, secret) = self.check_key()?;
        // Signature: Message signature using HMAC-SHA512 of (URI path + SHA256(nonce + POST data)) and base64 decoded secret API key
        let encoded = format!("{}{}", nonce, body);
        println!("encoded {}", encoded);

        let mut message = url.path().as_bytes().to_owned();
        message.extend(digest(&SHA256, encoded.as_bytes()).as_ref());

        println!("message {}", show(&message));

        let signed_key = hmac::Key::new(hmac::HMAC_SHA512, &b64decode(secret)?);
        let signature = hmac::sign(&signed_key, &message);
        let signature = b64encode(&signature);

        println!("Sig2 {}", signature);

        (key, signature)
    }

    #[throws(failure::Error)]
    async fn handle_response<T: DeserializeOwned>(&self, resp: Response) -> T {
        let resp = resp.text().await?;
        if let Ok(p) = from_str::<KrakenResponse<T>>(&resp) {
            return p.result;
        }

        if let Ok(p) = from_str::<KrakenErrorResponse>(&resp) {
            throw!(KrakenError::from(p))
        }

        error!("Cannot deserialize {}", resp);
        throw!(KrakenError::CannotDeserializeResponse(resp));
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
