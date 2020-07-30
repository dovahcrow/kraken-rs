use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum All {
    #[serde(rename = "all")]
    All,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Challenge {
    #[serde(rename = "challenge")]
    Challenge,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Info {
    #[serde(rename = "info")]
    Info,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Subscribed {
    #[serde(rename = "subscribed")]
    Subscribed,
}
