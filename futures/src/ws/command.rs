use crate::Symbol;
use fehler::{throw, throws};
use serde::{ser, Deserialize, Serialize, Serializer};
use serde_json::json;
use url::Url;

pub enum Command {
    AccountBalance,
    Book { product_ids: Vec<Symbol> },
    Challenge,
    Fills,
    Heartbeat,
    Trade { product_ids: Vec<Symbol> },
    Ping,
    Pong,
}

impl Command {
    pub fn account_balance() -> Self {
        Self::AccountBalance
    }

    pub fn book(product_ids: &[Symbol]) -> Self {
        Self::Book { product_ids: product_ids.into() }
    }

    pub fn challenge() -> Self {
        Self::Challenge
    }

    pub fn fills() -> Self {
        Self::Fills
    }

    pub fn heartbeat() -> Self {
        Self::Heartbeat
    }

    #[throws(failure::Error)]
    pub fn trade(product_ids: &[&str]) -> Self {
        Self::Trade {
            product_ids: product_ids.iter().map(|s| s.parse()).collect::<Result<Vec<_>, _>>()?,
        }
    }
}

impl Serialize for Command {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let v = match self {
            Self::AccountBalance => json!({
                "event":"subscribe",
                "feed": "account_balances_and_margins",
            }),
            Self::Book { product_ids } => json!({
               "event": "subscribe",
               "feed": "book",
               "product_ids": product_ids
            }),
            Self::Challenge => throw!(ser::Error::custom("Cannot serialize challenge to Kraken Websocket Message")),
            Self::Fills => json!({
                "event":"subscribe",
                "feed": "fills",
            }),
            Self::Heartbeat => json!({
               "event": "subscribe",
               "feed": "heartbeat",
            }),
            Self::Trade { product_ids } => json!({
               "event": "subscribe",
               "feed": "trade",
               "product_ids": product_ids
            }),
            Self::Ping => throw!(ser::Error::custom("Cannot serialize Ping to Kraken Websocket Message")),
            Self::Pong => throw!(ser::Error::custom("Cannot serialize Pong to Kraken Websocket Message")),
        };

        v.serialize(serializer)
    }
}
