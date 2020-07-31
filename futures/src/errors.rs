use std::backtrace::Backtrace;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KrakenError {
    #[error("No Api key set for private api")]
    NoApiKeySet,
    #[error("Kraken error {0:?}")]
    KrakenError(String),
    #[error("Cannot deserialize response {0}")]
    CannotDeserializeResponse(String),
    #[error("Websocket closed")]
    WebsocketClosed,
    #[error("Unexpected websocket binary content {0:?}")]
    UnexpectedWebsocketBinaryContent(Vec<u8>),
    #[error("Failed to parse pair {0}")]
    ParsePairFailed(String),
    #[error("Failed to parse symbol {0}")]
    ParseSymbolFailed(String),
    #[error("Failed to parse currency {0}")]
    ParseCurrencyFailed(String),

    // Foreign errors
    #[error("IO error {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error("Websocket error {source}")]
    Websocket {
        #[from]
        source: tungstenite::error::Error,
    },
    #[error("Json error {source}")]
    Json {
        #[from]
        source: serde_json::error::Error,
    },
    #[error("Base64 error {source}")]
    Base64 {
        #[from]
        source: base64::DecodeError,
        backtrace: Backtrace,
    },
    #[error("HTTP error {source}")]
    Http {
        #[from]
        source: reqwest::Error,
    },
    #[error("URLEncode error {source}")]
    URLEncode {
        #[from]
        source: serde_urlencoded::ser::Error,
    },
    #[error("URL parse error {source}")]
    UrlParse {
        #[from]
        source: url::ParseError,
    },
}
