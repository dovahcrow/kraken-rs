use chrono::{DateTime, Utc};
use failure::Fail;
use serde::{Deserialize, Deserializer};
use serde_json::Value;

#[derive(Fail, Debug)]
pub enum KrakenError {
    #[fail(display = "No Api key set for private api")]
    NoApiKeySet,
    #[fail(display = "Kraken error {:?}", _0)]
    KrakenError(String),
    #[fail(display = "Cannot deserialize response {}", _0)]
    CannotDeserializeResponse(String),
    #[fail(display = "Websocket closed")]
    WebsocketClosed,
    #[fail(display = "Unexpected websocket binary content {:?}", _0)]
    UnexpectedWebsocketBinaryContent(Vec<u8>),
    #[fail(display = "Failed to parse pair {}", _0)]
    ParsePairFailed(String),
    #[fail(display = "Failed to parse symbol {}", _0)]
    ParseSymbolFailed(String),
}

impl From<KrakenErrorResponse> for KrakenError {
    fn from(error: KrakenErrorResponse) -> KrakenError {
        KrakenError::KrakenError(error.error)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct KrakenResponse<T> {
    result: Success,
    #[serde(rename = "serverTime")]
    pub(crate) server_time: DateTime<Utc>,
    #[serde(flatten)]
    pub(crate) payload: T,
}

// The error response from bitmex;
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct KrakenErrorResponse {
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

// pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let s = String::deserialize(deserializer)?;

//     DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom).map(|d| d.into())
// }
