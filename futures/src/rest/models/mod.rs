mod accounts;
mod cancel_all_orders;
mod cancel_order;
mod open_positions;
mod orderbook;
mod send_order;
mod tickers;

pub use accounts::{AccountDetail, AccountsRequest, AccountsResponse, Auxiliary, MarginRequirements};
pub use cancel_all_orders::{CancelAllOrdersRequest, CancelAllOrdersResponse, CancelledOrder};
pub use cancel_order::{CancelOrderRequest, CancelOrderResponse, CancelStatus, Status};
pub use open_positions::{OpenPosition, OpenPositionsRequest, OpenPositionsResponse};
pub use orderbook::{OrderbookRequest, OrderbookResponse};
pub use send_order::{SendOrderRequest, SendOrderResponse, SendStatus};
pub use tickers::{TickersRequest, TickersResponse};

use http::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait Request: Serialize {
    const METHOD: Method;
    const SIGNED: bool = false;
    const ENDPOINT: &'static str;
    const HAS_PAYLOAD: bool = true;
    type Response: DeserializeOwned;

    #[inline]
    fn no_payload(&self) -> bool {
        !Self::HAS_PAYLOAD
    }
}
