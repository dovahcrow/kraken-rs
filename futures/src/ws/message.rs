use crate::common::{constants, FillType, Side, Symbol};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Message {
    Subscribed {
        event: constants::Subscribed,
        feed: String,
        #[serde(flatten)]
        extra: HashMap<String, Value>,
    },
    Info {
        event: constants::Info,
        version: i64,
    },
    Challenge {
        event: constants::Challenge,
        message: String,
    },
    Subscription(SubscriptionMessage), // Subscriptions don't have the event field
    Ping,
    Pong,
}

// Bellow are structs for Subscriptions
#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "feed", rename_all = "snake_case")]
pub enum SubscriptionMessage {
    AccountBalance {
        seq: u64,
        margin_accounts: Vec<MarginAccount>,
        account: String,
    },
    Book {
        product_id: Symbol,
        timestamp: i64,
        side: Side,
        seq: u64,
        price: f64,
        qty: f64,
    },
    BookSnapshot {
        product_id: Symbol,
        timestamp: i64,
        seq: u64,
        #[serde(default)]
        bids: Vec<PriceTuple>,
        #[serde(default)]
        asks: Vec<PriceTuple>,
    },
    Fills {
        username: String,
        fills: Vec<SingleFill>,
    },
    FillsSnapshot {
        account: String,
        fills: Vec<SingleFill>,
    },
    Heartbeat {
        time: u64,
    },
}

#[derive(Debug, Deserialize, Clone)]
pub struct MarginAccount {
    pub name: String,
    pub pv: f64,
    pub balance: f64,
    pub funding: f64,
    pub mm: f64,
    pub pnl: f64,
    pub im: f64,
    pub am: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PriceTuple {
    pub price: f64,
    pub qty: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SingleFill {
    pub instrument: Symbol,
    pub time: u64,
    pub price: f64,
    pub seq: u64,
    pub buy: bool,
    pub qty: f64,
    pub order_id: Uuid,
    pub cli_ord_id: Option<Uuid>,
    pub fill_id: Uuid,
    pub fill_type: FillType,
}

// impl<'de> Deserialize<'de> for Message {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let mut pm = Message::deserialize(deserializer)?;

//         match pm {
//             Message::Info { version, .. } => Ok(Self::Info(version)),
//             Message::Subscribed { feed, mut extra, .. } => match feed.as_str() {
//                 "book" => {
//                     let product_ids = extra.remove("product_ids").ok_or(serde::de::Error::missing_field("product_ids"))?;
//                     let product_ids: Vec<Symbol> = from_value(product_ids).map_err(serde::de::Error::custom)?;

//                     Ok(Self::SubscribedBook(product_ids))
//                 }
//                 "fills" => Ok(Self::SubscribedFills),
//                 event => {
//                     unimplemented!("Unimplemented");
//                 }
//             },
//             Message::Subscription(v) => Ok(Self::BookSnapshot(from_value(v).map_err(serde::de::Error::custom)?)),
//             Message::Challenge { message, .. } => Ok(Self::Challenge(message)),
//         }
//     }
// }
