use crate::errors::KrakenError;
use fehler::{throw, throws};
use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{from_str, to_string};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Symbol {
    Cash,
    FutureInverse(Pair, Option<String>),
    FutureVanilla(Pair, Option<String>),
    PerpetualInverse(Pair),
    PerpetualVanilla(Pair),
    Index(Pair),
    ReferenceRate(Pair),
}

impl FromStr for Symbol {
    type Err = failure::Error;
    #[throws(failure::Error)]
    fn from_str(s: &str) -> Symbol {
        from_str(&format!("\"{}\"", s))?
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", to_string(&self).map_err(|_| fmt::Error)?)
    }
}

impl std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Serialize for Symbol {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let repr = match self {
            Self::Cash => "CASH".to_string(),
            Self::FutureInverse(pair, expire) => {
                if let Some(expire) = expire {
                    format!("FI_{}_{}", pair, expire)
                } else {
                    format!("FI_{}", pair)
                }
            }
            Self::FutureVanilla(pair, expire) => {
                if let Some(expire) = expire {
                    format!("FV_{}_{}", pair, expire)
                } else {
                    format!("FV_{}", pair)
                }
            }
            Self::PerpetualInverse(pair) => format!("PI_{}", pair),
            Self::PerpetualVanilla(pair) => format!("PV_{}", pair),
            Self::Index(pair) => format!("IN_{}", pair),
            Self::ReferenceRate(pair) => format!("RR_{}", pair),
        };

        serializer.serialize_str(&repr)
    }
}

impl<'de> Deserialize<'de> for Symbol {
    #[throws(D::Error)]
    fn deserialize<D>(deserialize: D) -> Self
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserialize)?.to_uppercase();

        if s == "CASH" {
            return Self::Cash;
        }

        let pieces: Vec<_> = s.split("_").collect();

        match &pieces[..] {
            ["FI", pair, expire] => Symbol::FutureInverse(pair.parse().map_err(DeError::custom)?, Some(expire.to_string())),
            ["FV", pair, expire] => Symbol::FutureVanilla(pair.parse().map_err(DeError::custom)?, Some(expire.to_string())),
            ["FI", pair] => Symbol::FutureInverse(pair.parse().map_err(DeError::custom)?, None),
            ["FV", pair] => Symbol::FutureVanilla(pair.parse().map_err(DeError::custom)?, None),
            ["PI", pair] => Symbol::PerpetualInverse(pair.parse().map_err(DeError::custom)?),
            ["PV", pair] => Symbol::PerpetualVanilla(pair.parse().map_err(DeError::custom)?),
            ["IN", pair] => Symbol::Index(pair.parse().map_err(DeError::custom)?),
            ["RR", pair] => Symbol::ReferenceRate(pair.parse().map_err(DeError::custom)?),
            _ => throw!(DeError::custom(KrakenError::ParseSymbolFailed(s))),
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pair(String, String);

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl std::fmt::Debug for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl FromStr for Pair {
    type Err = failure::Error;
    #[throws(failure::Error)]
    fn from_str(s: &str) -> Pair {
        from_str(&format!("\"{}\"", s))?
    }
}

impl<'de> Deserialize<'de> for Pair {
    #[throws(D::Error)]
    fn deserialize<D>(deserialize: D) -> Self
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserialize)?.to_uppercase();
        if s.len() != 6 {
            throw!(DeError::custom(KrakenError::ParsePairFailed(s)));
        }
        Self(s[..3].into(), s[3..].into())
    }
}

impl Serialize for Pair {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}{}", self.0, self.1))
    }
}