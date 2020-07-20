use super::Request;
use crate::{Side, Symbol};
use chrono::{DateTime, Utc};
use http::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Debug, Clone)]
pub struct AccountsRequest;

#[derive(Deserialize, Debug, Clone)]
pub struct AccountsResponse {
    pub accounts: HashMap<Symbol, AccountDetail>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AccountDetail {
    pub r#type: String,
    pub currency: Option<String>,
    pub balances: HashMap<String, f64>,
    pub auxiliary: Option<Auxiliary>,
    #[serde(rename = "marginRequirements")]
    pub margin_requirements: Option<MarginRequirements>,
    #[serde(rename = "triggerEstimates")]
    pub trigger_estimates: Option<MarginRequirements>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Auxiliary {
    #[serde(rename = "af")]
    pub af: f64,

    #[serde(rename = "pnl")]
    pub pnl: f64,

    #[serde(rename = "pv")]
    pub pv: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MarginRequirements {
    #[serde(rename = "im")]
    pub im: f64,

    #[serde(rename = "mm")]
    pub mm: f64,

    #[serde(rename = "lt")]
    pub lt: f64,

    #[serde(rename = "tt")]
    pub tt: f64,
}

impl Request for AccountsRequest {
    const METHOD: Method = Method::GET;
    const SIGNED: bool = true;
    const ENDPOINT: &'static str = "/accounts";
    const HAS_PAYLOAD: bool = false;
    type Response = AccountsResponse;
}
