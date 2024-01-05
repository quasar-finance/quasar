use cosmwasm_std::{StdError, Uint128};
use cw_asset::AssetError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AirdropErrors {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Asset(#[from] AssetError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid airdrop window")]
    InvalidAirdropWindow {},

    #[error("Airdrop config cannot be changed once airdrop is active")]
    InvalidChangeInConfig {},

    #[error("User info cannot be changed once airdrop is active")]
    InvalidChangeUserInfo {},

    #[error("Withdraw is in an invalid window")]
    InvalidWithdraw {},

    #[error("Claim is in an invalid window")]
    InvalidClaim {},

    #[error("Already claimed")]
    AlreadyClaimed {},

    #[error("Already exists : {user:?}")]
    AlreadyExists { user: String },

    #[error("Invalid change in airdrop config")]
    InvalidChangeInAirdropConfig {},

    #[error("Insufficient funds in contract account. Balance: {balance:?}")]
    InsufficientFundsInContractAccount { balance: Uint128 },

    #[error("Total amount in the given user amounts {total_in_user_info:?} is greater than {current_airdrop_amount:?}")]
    UserAmountIsGreaterThanTotal {
        total_in_user_info: Uint128,
        current_airdrop_amount: Uint128,
    },

    #[error("Failed due to config has less amount than the amount allowed to the users to claim")]
    ConfigAmountLessThanTotalClaimable {},

    #[error("Failed as total claimed is non zero")]
    NonZeroClaimedAmount {},

    #[error("Given number of users do not match the given number of amounts which results into a mismatch")]
    UnequalLengths {},

    #[error("Amount for address {address:?} is zero")]
    ZeroAmount { address: String },
}
