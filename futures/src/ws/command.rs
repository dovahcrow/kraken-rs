use crate::Symbol;
use fehler::throws;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::json;
use url::Url;

pub enum Command {
    Fills,
    Book { product_ids: Vec<Symbol> },
    Trade { product_ids: Vec<Symbol> },
    Challenge { key: String },
    Heatbeat,
}

impl Command {
    pub fn fills() -> Self {
        Self::Fills
    }
    #[throws(failure::Error)]
    pub fn book(product_ids: &[&str]) -> Self {
        Self::Book {
            product_ids: product_ids.iter().map(|s| s.parse()).collect::<Result<Vec<_>, _>>()?,
        }
    }

    #[throws(failure::Error)]
    pub fn trade(product_ids: &[&str]) -> Self {
        Self::Trade {
            product_ids: product_ids.iter().map(|s| s.parse()).collect::<Result<Vec<_>, _>>()?,
        }
    }

    pub fn challenge(key: &str) -> Self {
        Self::Challenge { key: key.into() }
    }

    pub fn heatbeat() -> Self {
        Self::Heatbeat
    }
}

impl Serialize for Command {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let v = match self {
            Self::Fills => json!({
                "event":"subscribe",
                "feed": "fills",
            }),
            Self::Book { product_ids } => json!({
               "event": "subscribe",
               "feed": "book",
               "product_ids": product_ids
            }),
            Self::Trade { product_ids } => json!({
               "event": "subscribe",
               "feed": "trade",
               "product_ids": product_ids
            }),
            Self::Challenge { key } => json!({
               "event": "challenge",
               "api_key": key,
            }),
            Self::Heatbeat => json!({
               "event": "subscribe",
               "feed": "heartbeat",
            }),
        };

        v.serialize(serializer)
    }
}
