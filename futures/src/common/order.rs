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

impl PartialEq for Order {
    fn eq(&self, other: &Order) -> bool {
        self.order_id == other.order_id
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum OrderEvent {
    #[serde(rename_all = "camelCase")]
    Execution {
        execution_id: Uuid,
        price: f64,
        amount: f64,
        order_prior_edit: Option<Order>,
        order_prior_execution: Order,
    },
    NewOrder {
        order: Order,
        reason: Option<String>,
        r#type: String,
    },
}

impl OrderEvent {
    pub fn order_id(&self) -> Uuid {
        match self {
            OrderEvent::Execution { order_prior_execution, .. } => order_prior_execution.order_id,
            OrderEvent::NewOrder { order, .. } => order.order_id,
        }
    }
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
