use super::Request;
use crate::common::{Currency, Either, Symbol};
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
#[serde(tag = "type", rename_all = "camelCase")]
pub enum AccountDetail {
    CashAccount {
        balances: HashMap<Either<Currency, Symbol>, f64>,
    },
    #[serde(rename_all = "camelCase")]
    MarginAccount {
        currency: Currency,
        balances: HashMap<Either<Currency, Symbol>, f64>,
        auxiliary: Auxiliary,
        margin_requirements: MarginRequirements,
        trigger_estimates: MarginRequirements,
    },
}

#[derive(Debug, Deserialize, Clone)]
pub struct Auxiliary {
    pub af: f64,
    pub pnl: f64,
    pub pv: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MarginRequirements {
    pub im: f64,
    pub mm: f64,
    pub lt: f64,
    pub tt: f64,
}

impl Request for AccountsRequest {
    const METHOD: Method = Method::GET;
    const SIGNED: bool = true;
    const ENDPOINT: &'static str = "/accounts";
    const HAS_PAYLOAD: bool = false;
    type Response = AccountsResponse;
}
