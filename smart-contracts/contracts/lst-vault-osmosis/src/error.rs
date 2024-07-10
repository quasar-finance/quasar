use cosmwasm_std::{OverflowError, StdError};
use mars_owner::OwnerError;
use quasar_types::error::FundsError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Nothing to claim.")]
    NothingToClaim {},

    #[error("Insufficient funds.")]
    InsufficientFunds {},

    #[error("{0}")]
    Funds(#[from] FundsError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("{0}")]
    Owner(#[from] OwnerError),
}
