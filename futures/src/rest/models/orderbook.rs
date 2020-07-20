use super::Request;
use crate::Symbol;
use http::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Debug, Clone)]
pub struct OrderbookRequest {
    pub symbol: Symbol,
}

impl Orderbook {
    pub fn new(symbol: Symbol) -> OrderbookRequest {
        OrderbookRequest { symbol }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct OrderbookResponse {
    #[serde(rename = "orderBook")]
    pub order_book: Orderbook,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Orderbook {
    pub bids: Vec<(f64, f64)>,
    pub asks: Vec<(f64, f64)>,
}

impl Request for OrderbookRequest {
    const METHOD: Method = Method::GET;
    const SIGNED: bool = false;
    const ENDPOINT: &'static str = "/orderbook";
    const HAS_PAYLOAD: bool = true;
    type Response = OrderbookResponse;
}
