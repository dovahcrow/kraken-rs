use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum FillType {
    Maker,
    Taker,
    Liquidation,
    Assignee,
    Assignor,
    TakerAfterEdit,
}
