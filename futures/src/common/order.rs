use super::{Side, Symbol};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub order_id: Uuid,
    pub cli_ord_id: Option<Uuid>,
    pub r#type: String,
    pub symbol: Symbol,
    pub side: Side,
    pub quantity: i64,
    pub filled: i64,
    pub limit_price: f64,
    pub stop_price: Option<f64>,
    pub reduce_only: bool,
    pub timestamp: DateTime<Utc>,
    pub last_update_timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OrderEvent {
    pub uid: Option<Uuid>,
    pub order: Order,
    pub reason: Option<String>,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderType {
    // lmt for a limit order
    #[serde(rename = "lmt")]
    Lmt,
    // post for a post-only limit order
    #[serde(rename = "post")]
    Post,
    // stp for a stop order
    #[serde(rename = "stp")]
    Stp,
    // take_profit for a take profit order
    #[serde(rename = "take_profit")]
    TakeProfit,
    // ioc for an immediate-or-cancel order
    #[serde(rename = "ioc")]
    IOC,
}
