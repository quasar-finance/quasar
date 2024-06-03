use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OwnershipError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Ownership proposal already exists")]
    ProposalAlreadyExists,

    #[error("No ownership proposal exists")]
    NoProposalExists,

    #[error("Invalid ownership proposal")]
    InvalidProposal,
}
