use failure::Fail;

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
