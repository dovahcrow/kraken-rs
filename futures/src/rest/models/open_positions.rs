use super::Request;
use crate::common::{Order, OrderEvent, OrderType, Pair, PositionSide, SendOrderStatus, Side, Symbol, TriggerSignal};
use chrono::{DateTime, Utc};
use http::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
pub struct OpenPositionsRequest;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpenPositionsResponse {
    pub open_positions: Vec<OpenPosition>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpenPosition {
    side: PositionSide,
    symbol: Symbol,
    price: f64,
    fill_time: DateTime<Utc>,
    size: f64,
    unrealized_funding: Option<f64>,
}

impl Request for OpenPositionsRequest {
    const METHOD: Method = Method::GET;
    const SIGNED: bool = true;
    const ENDPOINT: &'static str = "/openpositions";
    const HAS_PAYLOAD: bool = false;
    type Response = OpenPositionsResponse;
}
