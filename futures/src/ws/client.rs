use super::command::Command;
use super::message::Message as KrakenWsMessage;
use crate::errors::KrakenError;
use base64::{decode as b64decode, encode as b64encode};
use failure::Fallible;
use fehler::{throw, throws};
use futures::sink::Sink;
use futures::stream::Stream;
use futures::task::{Context, Poll};
use log::trace;
use pin_project::pin_project;
use ring::digest::{digest, SHA256};
use ring::hmac;
use serde::Serialize;
use serde_json::{from_str, json, to_string};
use std::pin::Pin;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tungstenite::protocol::Message as WSMessage;
use url::Url;

const WS_URL: &'static str = "wss://futures.kraken.com/ws/v1";

#[allow(dead_code)]
type WSStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct KrakenWebsocket {
    inner: WSStream,
    credential: Option<(String, String)>,
}

impl KrakenWebsocket {
    #[throws(failure::Error)]
    pub async fn new() -> Self {
        let (stream, _) = connect_async(Url::parse(&WS_URL).unwrap()).await?;

        Self { inner: stream, credential: None }
    }

    #[throws(failure::Error)]
    pub async fn with_credential(api_key: &str, api_secret: &str) -> Self {
        let (stream, _) = connect_async(Url::parse(&WS_URL).unwrap()).await?;

        Self {
            inner: stream,
            credential: Some((api_key.into(), api_secret.into())),
        }
    }

    #[throws(failure::Error)]
    fn check_key(&self) -> (&str, &str) {
        match self.credential.as_ref() {
            None => throw!(KrakenError::NoApiKeySet),
            Some((k, s)) => (k.as_str(), s.as_str()),
        }
    }

    #[throws(failure::Error)]
    pub fn signature(&self, challenge: &str) -> (&str, String) {
        let (key, secret) = self.check_key()?;

        // Hash the challenge with the SHA-256 algorithm
        // Base64-decode your api_secret
        // Use the result of step 2 to hash the result of step 1 with the HMAC-SHA-512 algorithm
        // Base64-encode the result of step 3

        let digest = digest(&SHA256, challenge.as_bytes());
        let signed_key = hmac::Key::new(hmac::HMAC_SHA512, &b64decode(secret)?);
        let signature = hmac::sign(&signed_key, digest.as_ref());
        let signature = b64encode(&signature);

        (key, signature)
    }
}

impl Sink<Command> for KrakenWebsocket {
    type Error = failure::Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let inner = Pin::new(&mut self.inner);
        inner.poll_ready(cx).map_err(|e| e.into())
    }

    fn start_send(mut self: Pin<&mut Self>, item: Command) -> Result<(), Self::Error> {
        let key = self.check_key()?.0.to_string();
        let inner = Pin::new(&mut self.inner);
        let command = match item {
            Command::Challenge => to_string(&json!({
               "event": "challenge",
               "api_key": key,
            }))?,
            item => to_string(&item)?,
        };

        trace!("Sending '{}' through websocket", command);
        Ok(inner.start_send(WSMessage::Text(command))?)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let inner = Pin::new(&mut self.inner);
        inner.poll_flush(cx).map_err(|e| e.into())
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        let inner = Pin::new(&mut self.inner);
        inner.poll_close(cx).map_err(|e| e.into())
    }
}

#[derive(Serialize)]
struct ExtendedPrivateCommand<T> {
    #[serde(flatten)]
    command: T,
    api_key: String,
    original_challenge: String,
    signed_challenge: String,
}

impl<U: AsRef<str>> Sink<(Command, U)> for KrakenWebsocket {
    type Error = failure::Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        <Self as Sink<Command>>::poll_ready(self, cx)
    }

    fn start_send(mut self: Pin<&mut Self>, (command, challenge): (Command, U)) -> Result<(), Self::Error> {
        let (api_key, sig) = self.signature(challenge.as_ref())?;
        let api_key = api_key.to_string();

        let inner = Pin::new(&mut self.inner);

        let command = ExtendedPrivateCommand {
            command,
            api_key,
            original_challenge: challenge.as_ref().into(),
            signed_challenge: sig,
        };
        let command = to_string(&command)?;
        trace!("Sending '{}' through websocket", command);
        Ok(inner.start_send(WSMessage::Text(command))?)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        <Self as Sink<Command>>::poll_flush(self, cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        <Self as Sink<Command>>::poll_close(self, cx)
    }
}

impl Stream for KrakenWebsocket {
    type Item = Fallible<KrakenWsMessage>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let inner = Pin::new(&mut self.inner);
        let poll = inner.poll_next(cx);
        match poll {
            Poll::Ready(Some(Err(e))) => Poll::Ready(Some(Err(e.into()))),
            Poll::Ready(Some(Ok(m))) => match parse_message(m) {
                Ok(m) => Poll::Ready(Some(Ok(m))),
                Err(e) => Poll::Ready(Some(Err(e))),
            },
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[throws(failure::Error)]
fn parse_message(msg: WSMessage) -> KrakenWsMessage {
    match msg {
        WSMessage::Text(message) => match message.as_str() {
            others => match from_str(others) {
                Ok(r) => r,
                Err(e) => unreachable!("Cannot deserialize message '{}' from Kraken: {}", others, e),
            },
        },
        WSMessage::Close(_) => throw!(KrakenError::WebsocketClosed),
        WSMessage::Binary(c) => throw!(KrakenError::UnexpectedWebsocketBinaryContent(c)),
        WSMessage::Ping(_) => KrakenWsMessage::Ping,
        WSMessage::Pong(_) => KrakenWsMessage::Pong,
    }
}
