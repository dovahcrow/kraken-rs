use crate::errors::KrakenError;
use fehler::{throw, throws};
use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::from_str;
use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash, Deserialize, Serialize)]
pub enum Currency {
    #[serde(alias = "xbt")]
    XBT,
    #[serde(alias = "eth")]
    ETH,
    #[serde(alias = "bch")]
    BCH,
    #[serde(alias = "xrp")]
    XRP,
    #[serde(alias = "ltc")]
    LTC,
    #[serde(alias = "usd")]
    USD,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Currency {
    type Err = KrakenError;

    #[throws(KrakenError)]
    fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "XBT" => Currency::XBT,
            "ETH" => Currency::ETH,
            "BCH" => Currency::BCH,
            "XRP" => Currency::XRP,
            "LTC" => Currency::LTC,
            "USD" => Currency::USD,
            _ => throw!(KrakenError::ParseCurrencyFailed(s.into())),
        }
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub enum Symbol {
    Cash,
    FutureInverse(Pair, Option<u64>),
    FutureVanilla(Pair, Option<u64>),
    PerpetualInverse(Pair),
    PerpetualVanilla(Pair),
    Index(Pair),
    ReferenceRate(Pair),
}

impl Symbol {
    pub fn pair(&self) -> Option<Pair> {
        match self {
            Symbol::Cash => None,
            Symbol::FutureInverse(p, _) => Some(*p),
            Symbol::FutureVanilla(p, _) => Some(*p),
            Symbol::PerpetualInverse(p) => Some(*p),
            Symbol::PerpetualVanilla(p) => Some(*p),
            Symbol::Index(p) => Some(*p),
            Symbol::ReferenceRate(p) => Some(*p),
        }
    }
}

impl FromStr for Symbol {
    type Err = KrakenError;
    #[throws(KrakenError)]
    fn from_str(s: &str) -> Symbol {
        from_str(&format!("\"{}\"", s))?
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::Cash => write!(f, "CASH"),
            Symbol::FutureInverse(p, None) => write!(f, "FI_{}", p),
            Symbol::FutureInverse(p, Some(e)) => write!(f, "FI_{}_{}", p, e),
            Symbol::FutureVanilla(p, None) => write!(f, "FV_{}", p),
            Symbol::FutureVanilla(p, Some(e)) => write!(f, "FV_{}_{}", p, e),
            Symbol::PerpetualInverse(p) => write!(f, "PI_{}", p),
            Symbol::PerpetualVanilla(p) => write!(f, "PV_{}", p),
            Symbol::Index(p) => write!(f, "IN_{}", p),
            Symbol::ReferenceRate(p) => write!(f, "RR_{}", p),
        }
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
        serializer.serialize_str(&self.to_string())
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
            ["FI", pair, expire] => Symbol::FutureInverse(pair.parse().map_err(DeError::custom)?, Some(expire.parse().map_err(DeError::custom)?)),
            ["FV", pair, expire] => Symbol::FutureVanilla(pair.parse().map_err(DeError::custom)?, Some(expire.parse().map_err(DeError::custom)?)),
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

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Copy)]
pub struct Pair(pub Currency, pub Currency);

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
    type Err = KrakenError;
    #[throws(KrakenError)]
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
        Self(s[..3].parse().map_err(DeError::custom)?, s[3..].parse().map_err(DeError::custom)?)
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
