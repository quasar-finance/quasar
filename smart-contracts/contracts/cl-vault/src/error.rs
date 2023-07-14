use apollo_cw_asset::AssetInfo;
use cosmwasm_std::{Coin, DivideByZeroError, OverflowError, StdError};
use cw_controllers::AdminError;
use cw_dex::CwDexError;
use cw_dex_router::ContractError as CwDexRouterError;
use cw_vault_token::CwTokenError;
use thiserror::Error;

/// AutocompoundingVault errors
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    CwDexError(#[from] CwDexError),

    #[error("{0}")]
    CwTokenError(#[from] CwTokenError),

    #[error("{0}")]
    CwDexRouterError(#[from] CwDexRouterError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("{0}")]
    AdminError(#[from] AdminError),

    #[error("{0}")]
    Cw20BaseError(#[from] cw20_base::ContractError),

    #[error("{0}")]
    DivideByZero(#[from] DivideByZeroError),

    #[error("{0}")]
    SemVer(#[from] semver::Error),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("invalid reply id: {id}; must be 1, 2 or 3")]
    InvalidReplyId { id: u64 },

    #[error("Invalid asset deposited.")]
    InvalidDepositAsset {},

    #[error("Invalid assets requested for withdrawal.")]
    InvalidWithdrawalAssets {},

    #[error("Invalid vault token deposited.")]
    InvalidVaultTokenDeposited {},

    #[error("Invalid base token.")]
    InvalidBaseToken {},

    #[error("asset field does not equal coins sent")]
    InvalidAssetField {},

    #[error("Invalid reward liquidation target. Reward liquidation target must be one of the pool assets. Expected one of: {expected:?}. Got {actual:?}")]
    InvalidRewardLiquidationTarget {
        expected: Vec<AssetInfo>,
        actual: AssetInfo,
    },

    #[error("Unknown reply ID: {0}")]
    UnknownReplyId(u64),

    #[error("Unexpected funds sent. Expected: {expected:?}, Actual: {actual:?}")]
    UnexpectedFunds {
        expected: Vec<Coin>,
        actual: Vec<Coin>,
    },

    #[error("No data in SubMsgResponse")]
    NoDataInSubMsgResponse {},

    #[error("{0}")]
    Generic(String),
}

impl From<String> for ContractError {
    fn from(val: String) -> Self {
        ContractError::Generic(val)
    }
}

impl From<&str> for ContractError {
    fn from(val: &str) -> Self {
        ContractError::Generic(val.into())
    }
}

impl From<ContractError> for StdError {
    fn from(e: ContractError) -> Self {
        StdError::generic_err(e.to_string())
    }
}
