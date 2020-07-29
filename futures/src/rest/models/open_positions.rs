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
    pub side: PositionSide,
    pub symbol: Symbol,
    pub price: f64,
    pub fill_time: DateTime<Utc>,
    pub size: f64,
    pub unrealized_funding: Option<f64>,
}

impl Request for OpenPositionsRequest {
    const METHOD: Method = Method::GET;
    const SIGNED: bool = true;
    const ENDPOINT: &'static str = "/openpositions";
    const HAS_PAYLOAD: bool = false;
    type Response = OpenPositionsResponse;
}
