mod constants;

use crate::Symbol;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{from_value, to_value, Value};
use std::collections::HashMap;

// Text("{\"success\":true,\"subscribe\":\"chat\",\"request\":{\"args\":[\"chat\"],\"op\":\"subscribe\"}}")
// Text("{\"table\":\"chat\",\"action\":\"insert\",\"keys\":[\"id\"],\"data\":[{\"channelID\":4,\"date\":\"2018-10-26T05:09:44.159Z\",\"fromBot\":false,\"html\":\"ㅋㅋㅋㅋㅋ ETF 드립 ㅈㄴ웃기네\\n\",\"id\":21699228,\"message\":\"ㅋㅋㅋㅋㅋ ETF 드립 ㅈㄴ웃기네\",\"user\":\"xixixiaqs\"}],\"filterKey\":\"channelID\"}")
// Text("{\"info\":\"Welcome to the BitMEX Realtime API.\",\"version\":\"2018-10-23T18:33:47.000Z\",\"timestamp\":\"2018-10-26T05:09:14.006Z\",\"docs\":\"https://www.bitmex.com/app/wsAPI\",\"limit\":{\"remaining\":38}}")
// {"success":true,"unsubscribe":"chat","request":{"op":"unsubscribe","args":["chat"]}}
// {"status":400,"error":"Failed to decode incoming data: Unexpected token a in JSON at position 0. Please see the documentation at https://www.bitmex.com/app/wsAPI.","meta":{}}

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
    BookSnapshot(BookSnapshot),
    Book(Book),
    FillsSnapshot(FillsSnapshot),
}

#[derive(Debug, Deserialize, Clone)]
pub struct FillsSnapshot {
    feed: constants::FillsSnapshot,
    pub account: String,
    pub fills: Vec<Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BookSnapshot {
    feed: constants::BookSnapshot,
    pub product_id: Symbol,
    pub timestamp: i64,
    pub seq: i64,
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
    pub seq: i64,
    #[serde(rename = "tickSize")]
    pub tick_size: Option<serde_json::Value>,
    pub price: f64,
    pub qty: f64,
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
