use super::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct GetTradeBalanceRequest {
    pub aclass: String,
    pub asset: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GetTradeBalanceResponse {
    pub eb: f64,
}

impl Request for GetTradeBalanceRequest {
    const SIGNED: bool = true;
    const ENDPOINT: &'static str = "/0/private/TradeBalance";
    const HAS_PAYLOAD: bool = true;
    type Response = Vec<GetTradeBalanceResponse>;
}
