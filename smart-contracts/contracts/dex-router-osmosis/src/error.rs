use cosmwasm_std::{Coin, OverflowError, StdError};
use mars_owner::OwnerError;
use quasar_types::error::FundsError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("{0}")]
    Owner(#[from] OwnerError),

    #[error("{0}")]
    Funds(#[from] FundsError),

    #[error("Invalid swap path: {path:?} {reason}")]
    InvalidSwapPath { path: Vec<u64>, reason: String },

    #[error("No path found for assets {offer:?} -> {ask:?}")]
    NoPathFound { offer: String, ask: String },

    #[error("No paths to check")]
    NoPathsToCheck {},

    #[error("Pool not found: {pool_id:?}")]
    PoolNotFound { pool_id: u64 },

    #[error("Can't set empty path.")]
    EmptyPath {},

    #[error("Computation of best path failed for swap from {offer:?} to {ask_denom:?}")]
    FailedBestPathComputation { offer: Coin, ask_denom: String },
}

pub fn assert_non_empty_path<T>(path: &[T]) -> Result<(), ContractError> {
    if path.is_empty() {
        return Err(ContractError::EmptyPath {});
    }
    Ok(())
}
