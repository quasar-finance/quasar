use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum BurnErrors {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Amount cannot be zero")]
    ZeroAmount {},
}
