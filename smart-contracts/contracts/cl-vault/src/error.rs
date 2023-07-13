use cosmwasm_std::{
    CheckedFromRatioError, CheckedMultiplyRatioError, DivideByZeroError, OverflowError, StdError,
};
use thiserror::Error;

/// AutocompoundingVault errors
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("{0}")]
    DivideByZeroError(#[from] DivideByZeroError),

    #[error("{0}")]
    CheckedMultiplyRatioError(#[from] CheckedMultiplyRatioError),

    #[error("Overflow")]
    Overflow {},

    #[error("{0}")]
    CwDexError(#[from] CwDexError),

    #[error("{0}")]
    MultiplyRatioError(#[from] CheckedFromRatioError),
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
