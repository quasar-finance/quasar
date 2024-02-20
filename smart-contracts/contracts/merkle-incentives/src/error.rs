use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Failed to decode root: {root:?}")]
    FailedToDecodeRoot { root: String },

    #[error("Failed to verify proof")]
    FailedVerifyProof {},

    #[error("Incentives already claimed")]
    IncentivesAlreadyClaimed {},

    #[error("Valid claim submitted but contract does not have enough balance, did the admin forget to top it up?")]
    InsufficientBalanceForValidClaim {},
}
