use cosmwasm_std::{
    CheckedFromRatioError, CheckedMultiplyRatioError, Coin, ConversionOverflowError,
    DivideByZeroError, OverflowError, StdError, Uint128,
};
use cw_dex::CwDexError;
use thiserror::Error;

/// AutocompoundingVault errors
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Position Not Found")]
    PositionNotFound,

    #[error("{0}")]
    DivideByZeroError(#[from] DivideByZeroError),

    #[error("{0}")]
    CheckedMultiplyRatioError(#[from] CheckedMultiplyRatioError),

    #[error("Overflow")]
    Overflow {},

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("{0}")]
    CwDexError(#[from] CwDexError),

    #[error("{0}")]
    ConversionOverflowError(#[from] ConversionOverflowError),

    #[error("{0}")]
    MultiplyRatioError(#[from] CheckedFromRatioError),

    #[error("This message does no accept funds")]
    NonPayable {},

    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.

    // todo: add apollo errors one by one and see what gives us type errors
    // apollo errors below (remove from above when you add)
    #[error("Unexpected funds sent. Expected: {expected:?}, Actual: {actual:?}")]
    UnexpectedFunds {
        expected: Vec<Coin>,
        actual: Vec<Coin>,
    },

    #[error("Bad token out requested for swap, must be one of: {base_token:?}, {quote_token:?}")]
    BadTokenForSwap {
        base_token: String,
        quote_token: String,
    },

    #[error("Insufficient funds for swap. Have: {balance}, Need: {needed}")]
    InsufficientFundsForSwap { balance: Uint128, needed: Uint128 },
}
