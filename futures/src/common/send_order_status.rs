use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum SendOrderStatus {
    // placed: the order was placed successfully
    Placed,
    // cancelled: the order was cancelled successfully
    Cancelled,
    // invalidOrderType: the order was not placed because orderType is invalid
    InvalidOrderType,
    // invalidSide: the order was not placed because side is invalid
    InvalidSide,
    // invalidSize: the order was not placed because size is invalid
    InvalidSize,
    // invalidPrice: the order was not placed because limitPrice and/or stopPrice are invalid
    InvalidPrice,
    // insufficientAvailableFunds: the order was not placed because available funds are insufficient
    InsufficientAvailableFunds,
    // selfFill: the order was not placed because it would be filled against an existing order belonging to the same account
    SelfFill,
    // tooManySmallOrders: the order was not placed because the number of small open orders would exceed the permissible limit
    TooManySmallOrders,
    // maxPositionViolation: Order would cause you to exceed your maximum position in this contract.
    MaxPositionViolation,
    // marketSuspended: the order was not placed because the market is suspended
    MarketSuspended,
    // marketInactive: the order was not placed because the market is inactive
    MarketInactive,
    // clientOrderIdAlreadyExist: the specified client id already exist
    ClientOrderIdAlreadyExist,
    // clientOrderIdTooLong: the client id is longer than the permissible limit
    ClientOrderIdTooLong,
    // outsidePriceCollar: the limit order crosses the spread but is an order of magnitude away from the mark price - fat finger control
    OutsidePriceCollar,
    // postWouldExecute: the post-only order would be filled upon placement, thus is cancelled
    PostWouldExecute,
    // iocWouldNotExecute: the immediate-or-cancel order would not execute.
    IocWouldNotExecute,
    // wouldCauseLiquidation: returned when a new order would fill at a worse price than the mark price, causing the portfolio value to fall below maintenance margin and triggering a liquidation.
    WouldCauseLiquidation,
    // wouldNotReducePosition: the reduce only order would not reduce position.
    WouldNotReducePosition,
}
