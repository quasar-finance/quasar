use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid merkle proof")]
    InvalidMerkleProof {},

    #[error("Incentives already claimed")]
    IncentivesAlreadyClaimed {},

    #[error("Valid claim submitted but contract does not have enough balance, did the admin forget to top it up?")]
    InsufficientBalanceForValidClaim {},
}
