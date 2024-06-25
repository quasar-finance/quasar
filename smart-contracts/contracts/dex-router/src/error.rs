use cosmwasm_std::{OverflowError, StdError};
use cw_asset::AssetError;
use mars_owner::OwnerError;
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;
use quasar_types::error::FundsError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("{0}")]
    Owner(#[from] OwnerError),

    #[error("{0}")]
    Funds(#[from] FundsError),

    #[error("{0}")]
    Asset(#[from] AssetError),

    #[error("Invalid swap path: {path:?} {reason}")]
    InvalidSwapPath {
        path: Vec<SwapAmountInRoute>,
        reason: String,
    },

    #[error("No path found for assets {offer:?} -> {ask:?}")]
    NoPathFound { offer: String, ask: String },

    #[error("No paths to check")]
    NoPathsToCheck {},
}
