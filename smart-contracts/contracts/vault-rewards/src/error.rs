use cosmwasm_std::{StdError, Uint128};
use cw_asset::AssetError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum VaultRewardsError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Asset(#[from] AssetError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid distribution schedule: {reason:?}")]
    InvalidDistributionSchedule { reason: String },

    #[error("No rewards to claim")]
    NoRewardsToClaim {},

    #[error("Insufficient funds in contract to process claim. Contract balance: {contract_balance:?}, Claim amount: {claim_amount:?}")]
    InsufficientFunds {
        contract_balance: Uint128,
        claim_amount: Uint128,
    },

    #[error("Invalid distribution schedule id. Max ID: {max_id:?}")]
    InvalidDistributionScheduleId { max_id: u64 },

    #[error(
        "Cannot edit distribution schedule in progress. ID: {id:?}, Start: {start:?}, End: {end:?}"
    )]
    DistributionScheduleInProgress { id: u64, start: u64, end: u64 },

    #[error("Cannot remove distribution schedule with funds left to be claimed. ID: {id:?}")]
    DistributionScheduleWithUnclaimedFunds { id: u64 },

    #[error("Cannot edit distribution schedule that has already ended. ID: {id:?}, End: {end:?}")]
    DistributionScheduleExpired { id: u64, end: u64 },

    // quasarypes or another name
    #[error("{0}")]
    QuasarError(#[from] quasar_types::error::Error),
}
