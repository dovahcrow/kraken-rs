use super::Request;
use crate::common::OrderEvent;
use http::Method;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug, Clone)]
pub struct CancelOrderRequest {
    order_id: Option<Uuid>,
    #[serde(rename = "cliOrdId")]
    cli_ord_id: Option<Uuid>,
}

impl CancelOrderRequest {
    pub fn from_order_id(u: Uuid) -> Self {
        Self {
            order_id: Some(u),
            cli_ord_id: None,
        }
    }

    pub fn from_cli_ord_id(u: Uuid) -> Self {
        Self {
            order_id: None,
            cli_ord_id: Some(u),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CancelOrderResponse {
    #[serde(rename = "cancelStatus")]
    pub cancel_status: CancelStatus,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CancelStatus {
    pub status: Status,
    #[serde(rename = "orderEvents", default)]
    pub order_events: Vec<OrderEvent>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Cancelled,
    Filled,
    NotFound,
}

impl Request for CancelOrderRequest {
    const METHOD: Method = Method::POST;
    const SIGNED: bool = true;
    const ENDPOINT: &'static str = "/cancelorder";
    const HAS_PAYLOAD: bool = true;
    type Response = CancelOrderResponse;
}
