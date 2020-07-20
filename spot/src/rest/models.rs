mod private;
mod public;

pub use private::{GetTradeBalanceRequest, GetTradeBalanceResponse};
pub use public::{GetServerTimeRequest, GetServerTimeResponse};

use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait Request: Serialize {
    const SIGNED: bool = false;
    const ENDPOINT: &'static str;
    const HAS_PAYLOAD: bool = true;
    type Response: DeserializeOwned;

    #[inline]
    fn no_payload(&self) -> bool {
        !Self::HAS_PAYLOAD
    }
}
