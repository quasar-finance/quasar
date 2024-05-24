// errors.rs
use cosmwasm_std::StdError;
use thiserror::Error;
use cw_controllers::AdminError;


#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("InvalidOwnership")]
    InvalidOwnership {},

    #[error("InvalidDuration({0})")]
    InvalidDuration(u64),

    #[error("ProposalNotFound")]
    ProposalNotFound {},

    #[error("Expired")]
    Expired {},

    #[error("InvalidFundsAmount")]
    InvalidFundsAmount{},

    #[error("{0}")]
    AdminError(#[from] AdminError),
}
