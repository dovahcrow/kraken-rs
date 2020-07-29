use serde::Deserialize;

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
