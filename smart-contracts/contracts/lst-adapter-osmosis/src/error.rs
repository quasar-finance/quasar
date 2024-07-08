use abstract_app::sdk::AbstractSdkError;
use abstract_app::std::AbstractError;
use abstract_app::AppError;
use cosmwasm_std::{Addr, CheckedMultiplyFractionError, OverflowError, StdError};
use mars_owner::OwnerError;
use quasar_types::error::FundsError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum LstAdapterError {
    #[error("{0}")]
    Abstract(#[from] AbstractError),

    #[error("{0}")]
    AbstractSdk(#[from] AbstractSdkError),

    #[error("{0}")]
    DappError(#[from] AppError),

    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("{0}")]
    CheckedMultiply(#[from] CheckedMultiplyFractionError),

    #[error("{0}")]
    Owner(#[from] OwnerError),

    #[error("{0}")]
    Funds(#[from] FundsError),

    #[error("{0}")]
    Json(String),

    #[error("Only configured vault can unbond or claim.")]
    NotVault {},

    #[error("Only configured observer can confirm transactions.")]
    NotObserver {},

    #[error("Missing remote address for {chain}")]
    MissingRemoteAddress { chain: String },

    #[error("Nothing to claim.")]
    NothingToClaim {},

    #[error("Unconfirmed unbond pending.")]
    UnconfirmedUnbondPending {},

    #[error("Nothing to confirm.")]
    NothingToConfirm {},

    #[error("No pending unbond.")]
    NoPendingUnbond {},

    #[error("Can't confirm unbond without funds being available.")]
    StillWaitingForFunds {},
}

pub fn assert_vault(sender: &Addr, vault: &Addr) -> Result<(), LstAdapterError> {
    if sender != vault {
        return Err(LstAdapterError::NotVault {});
    }
    Ok(())
}

pub fn assert_observer(sender: &Addr, observer: &Addr) -> Result<(), LstAdapterError> {
    if sender != observer {
        return Err(LstAdapterError::NotObserver {});
    }
    Ok(())
}
