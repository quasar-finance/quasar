use cosmwasm_std::StdError;
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

    #[error("Insufficient funds in contract account")]
    InsufficientFundsInContractAccount {},
}
