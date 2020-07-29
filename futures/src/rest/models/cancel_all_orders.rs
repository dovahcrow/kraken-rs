use super::Request;
use crate::common::{constants, Either, Order, OrderEvent, Side, Symbol};
use chrono::{DateTime, Utc};
use http::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
pub struct CancelAllOrdersRequest {
    symbol: Option<Symbol>,
}

impl CancelAllOrdersRequest {
    pub fn all() -> Self {
        Self { symbol: None }
    }

    pub fn by_symbol(symbol: Symbol) -> Self {
        Self { symbol: Some(symbol) }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersResponse {
    cancel_status: CancelStatus,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CancelStatus {
    pub received_time: DateTime<Utc>,
    pub cancel_only: Either<Symbol, constants::All>,
    pub status: CancelAllStatus,
    pub cancelled_orders: Vec<CancelledOrder>,
    pub order_events: Vec<OrderEvent>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum CancelAllStatus {
    Cancelled,
    NoOrdersToCancel,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CancelledOrder {
    #[serde(rename = "order_id")]
    pub order_id: Uuid,
}

impl Request for CancelAllOrdersRequest {
    const METHOD: Method = Method::POST;
    const SIGNED: bool = true;
    const ENDPOINT: &'static str = "/cancelallorders";
    const HAS_PAYLOAD: bool = true;
    type Response = CancelAllOrdersResponse;
}
