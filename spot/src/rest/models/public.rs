use super::Request;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Debug, Clone)]
pub struct GetServerTimeRequest;

#[derive(Deserialize, Debug, Clone)]
pub struct GetServerTimeResponse {
    pub unixtime: u64,
    #[serde(deserialize_with = "deserialize")]
    pub rfc1123: DateTime<Utc>,
}

impl Request for GetServerTimeRequest {
    const SIGNED: bool = false;
    const ENDPOINT: &'static str = "/0/public/Time";
    const HAS_PAYLOAD: bool = false;
    type Response = GetServerTimeResponse;
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    DateTime::parse_from_rfc2822(&s).map_err(serde::de::Error::custom).map(|d| d.into())
}
