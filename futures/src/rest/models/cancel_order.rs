use super::Request;
use crate::{Side, Symbol};
use chrono::{DateTime, Utc};
use http::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Debug, Clone)]
pub struct CancelOrderRequest {
    order_id: Option<String>,
    #[serde(rename = "cliOrdId")]
    cli_ord_id: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CancelOrderResponse {
    #[serde(rename = "cancelStatus")]
    pub cancel_status: CancelStatus,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CancelStatus {
    pub status: Status,
    #[serde(rename = "orderEvents")]
    pub order_events: Vec<OrderEvent>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OrderEvent {
    pub uid: String,
    pub order: Order,
    pub r#type: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Order {
    #[serde(rename = "orderId")]
    pub order_id: String,

    #[serde(rename = "cliOrdId")]
    pub cli_ord_id: Option<serde_json::Value>,
    pub r#type: String,
    pub symbol: Symbol,
    pub side: Side,
    pub quantity: i64,
    pub filled: i64,
    #[serde(rename = "limitPrice")]
    pub limit_price: i64,
    #[serde(rename = "stopPrice")]
    pub stop_price: Option<f64>,
    #[serde(rename = "reduceOnly")]
    pub reduce_only: bool,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Status {
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "filled")]
    Filled,
    #[serde(rename = "notFound")]
    NotFound,
}

impl Request for CancelOrderRequest {
    const METHOD: Method = Method::POST;
    const SIGNED: bool = true;
    const ENDPOINT: &'static str = "cancelorder";
    const HAS_PAYLOAD: bool = true;
    type Response = CancelOrderResponse;
}
