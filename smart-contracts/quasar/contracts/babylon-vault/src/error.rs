use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum VaultError {
    #[error("{0}")]
    Std(#[from] StdError),
}
