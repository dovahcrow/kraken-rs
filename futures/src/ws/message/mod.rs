mod constants;

use crate::common::{FillType, Side, Symbol};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{from_value, to_value, Value};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Deserialize)]
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
    Subscription(SubscriptionMessage), // Subscriptions don't have field event
    Ping,
    Pong,
}

// Bellow are structs for Subscriptions

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum SubscriptionMessage {
    AccountBalance(AccountBalance),
    Book(Book),
    BookSnapshot(BookSnapshot),
    Fills(Fills),
    FillsSnapshot(FillsSnapshot),
    Heartbeat(Heartbeat),
}

#[derive(Debug, Deserialize, Clone)]
pub struct AccountBalance {
    pub seq: u64,
    feed: constants::AccountBalancesAndMargins,
    pub margin_accounts: Vec<MarginAccount>,
    pub account: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MarginAccount {
    name: String,
    pv: f64,
    balance: f64,
    funding: f64,
    mm: f64,
    pnl: f64,
    im: f64,
    am: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BookSnapshot {
    feed: constants::BookSnapshot,
    pub product_id: Symbol,
    pub timestamp: i64,
    pub seq: u64,
    #[serde(rename = "tickSize")]
    pub tick_size: Option<serde_json::Value>,
    #[serde(default)]
    pub bids: Vec<PriceTuple>,
    #[serde(default)]
    pub asks: Vec<PriceTuple>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PriceTuple {
    pub price: f64,
    pub qty: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Book {
    feed: constants::Book,
    pub product_id: Symbol,
    pub timestamp: i64,
    pub side: Side,
    pub seq: u64,
    #[serde(rename = "tickSize")]
    pub tick_size: Option<serde_json::Value>,
    pub price: f64,
    pub qty: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Fills {
    feed: constants::Fills,
    pub username: String,
    pub fills: Vec<SingleFill>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FillsSnapshot {
    feed: constants::FillsSnapshot,
    pub account: String,
    pub fills: Vec<SingleFill>,
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

#[derive(Debug, Deserialize, Clone)]
pub struct Heartbeat {
    feed: constants::Heartbeat,
    time: u64,
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
