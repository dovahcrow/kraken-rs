use super::Request;
use http::Method;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct TickersRequest;

#[derive(Deserialize, Debug, Clone)]
pub struct TickersResponse {
    #[serde(rename = "tickers")]
    pub tickers: Vec<Ticker>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Ticker {
    pub tag: Option<String>,

    pub pair: Option<String>,

    pub symbol: String,

    #[serde(rename = "markPrice")]
    pub mark_price: Option<f64>,

    pub bid: Option<f64>,

    #[serde(rename = "bidSize")]
    pub bid_size: Option<i64>,

    pub ask: Option<f64>,

    #[serde(rename = "askSize")]
    pub ask_size: Option<i64>,

    #[serde(rename = "vol24h")]
    pub vol24_h: Option<i64>,

    #[serde(rename = "openInterest")]
    pub open_interest: Option<i64>,

    pub open24h: Option<f64>,

    pub last: f64,

    #[serde(rename = "lastTime")]
    pub last_time: String,

    #[serde(rename = "lastSize")]
    pub last_size: Option<i64>,

    pub suspended: Option<bool>,

    #[serde(rename = "fundingRate")]
    pub funding_rate: Option<f64>,

    #[serde(rename = "fundingRatePrediction")]
    pub funding_rate_prediction: Option<f64>,
}

impl Request for TickersRequest {
    const METHOD: Method = Method::GET;
    const SIGNED: bool = false;
    const ENDPOINT: &'static str = "/tickers";
    const HAS_PAYLOAD: bool = false;
    type Response = TickersResponse;
}
