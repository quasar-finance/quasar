use cosmwasm_std::StdError;
use thiserror::Error;

pub type ContractResult<T> = Result<T, ContractError>;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Destination already exists")]
    DestinationAlreadyExists,

    #[error("Destination does not exist")]
    DestinationNotExists,
}
