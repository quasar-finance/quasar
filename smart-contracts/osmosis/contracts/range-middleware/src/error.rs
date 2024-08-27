use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    /// start submission errors
    #[error("Invalid contract address {address}")]
    InvalidContractAddress { address: String },

    #[error("Query against cl-contract failed, maybe it isn't a CL contract?")]
    ClExpectedQueryFailed { address: String },

    // start execution errors
    #[error("No range exists for contract {address}")]
    NoRangeExists { address: String },
}
