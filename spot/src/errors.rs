use failure::Fail;
use serde::Deserialize;
use serde_json::Value;

#[derive(Fail, Debug)]
pub enum KrakenError {
    #[fail(display = "No Api key set for private api")]
    NoApiKeySet,
    #[fail(display = "Kraken error {:?}", _0)]
    KrakenError(Vec<Value>),
    #[fail(display = "Cannot deserialize response {:?}", _0)]
    CannotDeserializeResponse(String),
}

impl From<KrakenErrorResponse> for KrakenError {
    fn from(error: KrakenErrorResponse) -> KrakenError {
        KrakenError::KrakenError(error.error)
    }
}

// The error response from bitmex;
#[derive(Deserialize, Debug, Clone)]
pub(crate) struct KrakenErrorResponse {
    pub(crate) error: Vec<Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct KrakenResponse<T> {
    pub(crate) result: T,
}
