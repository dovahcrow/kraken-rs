pub mod constants;
mod fill_type;
mod order;
mod send_order_status;
mod side;
mod symbol;
mod trigger_signal;

pub use fill_type::FillType;
pub use order::{Order, OrderEvent, OrderType};
pub use send_order_status::SendOrderStatus;
pub use side::{PositionSide, Side};
pub use symbol::{Pair, Symbol};
pub use trigger_signal::TriggerSignal;

use serde::Deserialize;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
#[serde(untagged)]
pub enum Either<L, R> {
    /// A value of type `L`.
    Left(L),
    /// A value of type `R`.
    Right(R),
}
