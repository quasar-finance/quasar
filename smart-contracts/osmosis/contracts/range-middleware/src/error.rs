use cosmwasm_std::StdError;
use cw2::VersionError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Version(#[from] VersionError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid contract address {address}")]
    InvalidContractAddress { address: String },

    #[error("Query against cl-contract failed, maybe it isn't a CL contract?")]
    ClExpectedQueryFailed { address: String },

    #[error("No range exists for contract {address}")]
    NoRangeExists { address: String },
}
