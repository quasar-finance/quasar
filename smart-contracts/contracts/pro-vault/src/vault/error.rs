use thiserror::Error;

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("Strategy already exists")]
    StrategyAlreadyExists {},
}


/*

use cosmwasm_std::{StdError, StdResult};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VaultError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Strategy already exists")]
    StrategyAlreadyExists {},
}


impl From<VaultError> for StdError {
    fn from(err: VaultError) -> Self {
        match err {
            VaultError::Std(e) => e,
            _ => StdError::generic_err(err.to_string()),
        }
    }
}
*/
