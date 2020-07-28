use super::Request;
use crate::common::{Order, OrderEvent, Side, Status, Symbol};
use chrono::{DateTime, Utc};
use http::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
pub struct CancelAllOrderRequest {
    symbol: Option<Symbol>,
}

impl CancelAllOrderRequest {
    pub fn all() -> Self {
        Self { symbol: None }
    }

    pub fn by_symbol(symbol: &Symbol) -> Self {
        Self { symbol: Some(symbol.clone()) }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrderResponse {
    cancel_status: CancelStatus,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelStatus {
    pub received_time: DateTime<Utc>,
    pub cancel_only: String,
    pub status: Status,
    pub cancelled_orders: Vec<CancelledOrder>,
    pub order_events: Vec<OrderEvent>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CancelledOrder {
    #[serde(rename = "order_id")]
    pub order_id: Uuid,
}

impl Request for CancelAllOrderRequest {
    const METHOD: Method = Method::POST;
    const SIGNED: bool = true;
    const ENDPOINT: &'static str = "/cancelallorders";
    const HAS_PAYLOAD: bool = true;
    type Response = CancelAllOrderResponse;
}
