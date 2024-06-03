// errors.rs
use cosmwasm_std::StdError;
use thiserror::Error;
use cw_controllers::AdminError;
use crate::vault::error::VaultError; 

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

    #[error("{0}")]
    Vault(#[from] VaultError),

    #[error("Failed to set vault owner: {0}")]
    SetVaultOwnerError(String),

    #[error("Failed to save config: {0}")]
    SaveConfigError(String),

    #[error("Failed to update vault state: {0}")]
    UpdateVaultStateError(String),

    #[error("Admin and Vault Owner mismatch")]
    AdminVaultOwnerMismatch {},
}
