use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Price is not low enough.")]
    InvalidPrice {},

    #[error("Wrong offer denom.")]
    WrongDenom {},

    #[error("Missing or too many funds.")]
    InvalidFunds {},
}
