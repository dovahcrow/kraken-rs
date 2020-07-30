use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum All {
    #[serde(rename = "all")]
    All,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum AccountBalancesAndMargins {
    #[serde(rename = "account_balances_and_margins")]
    AccountBalancesAndMargins,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Book {
    #[serde(rename = "book")]
    Book,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum BookSnapshot {
    #[serde(rename = "book_snapshot")]
    BookSnapshot,
}

#[derive(Deserialize, Debug, Clone)]
pub enum CashAccount {
    #[serde(rename = "cashAccount")]
    CashAccount,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Challenge {
    #[serde(rename = "challenge")]
    Challenge,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Fills {
    #[serde(rename = "fills")]
    Fills,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum FillsSnapshot {
    #[serde(rename = "fills_snapshot")]
    FillsSnapshot,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Heartbeat {
    #[serde(rename = "heartbeat")]
    Heartbeat,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Info {
    #[serde(rename = "info")]
    Info,
}

#[derive(Deserialize, Debug, Clone)]
pub enum MarginAccount {
    #[serde(rename = "marginAccount")]
    MarginAccount,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Subscribed {
    #[serde(rename = "subscribed")]
    Subscribed,
}
