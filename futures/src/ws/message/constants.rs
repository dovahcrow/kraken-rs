use serde::Deserialize;

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

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum FillsSnapshot {
    #[serde(rename = "fills_snapshot")]
    FillsSnapshot,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum BookSnapshot {
    #[serde(rename = "book_snapshot")]
    BookSnapshot,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Book {
    #[serde(rename = "book")]
    Book,
}
