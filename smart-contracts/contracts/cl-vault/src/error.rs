// use apollo_cw_asset::AssetInfo;
// use cosmwasm_std::Coin;
// use cosmwasm_std::{
//     CheckedFromRatioError, CheckedMultiplyRatioError, DivideByZeroError, OverflowError, StdError,
// };
// use cw_controllers::AdminError;
// use cw_dex::CwDexError;
// use cw_dex_router::ContractError as CwDexRouterError;
// use cw_vault_token::CwTokenError;
// use thiserror::Error;

// /// AutocompoundingVault errors
// #[allow(missing_docs)]
// #[derive(Error, Debug)]
// pub enum ContractError {
//     #[error("{0}")]
//     Std(#[from] StdError),

//     #[error("Unauthorized")]
//     Unauthorized {},

//     #[error("{0}")]
//     CheckedMultiplyRatioError(#[from] CheckedMultiplyRatioError),

//     #[error("Overflow")]
//     Overflow {},

//     #[error("{0}")]
//     CwDexError(#[from] CwDexError),

//     #[error("{0}")]
//     CwTokenError(#[from] CwTokenError),

//     #[error("{0}")]
//     CwDexRouterError(#[from] CwDexRouterError),

//     #[error("{0}")]
//     AdminError(#[from] AdminError),

//     #[error("{0}")]
//     Cw20BaseError(#[from] cw20_base::ContractError),

//     #[error("{0}")]
//     DivideByZero(#[from] DivideByZeroError),

//     #[error("{0}")]
//     SemVer(#[from] semver::Error),

//     #[error("invalid reply id: {id}; must be 1, 2 or 3")]
//     InvalidReplyId { id: u64 },

//     #[error("Invalid asset deposited.")]
//     InvalidDepositAsset {},

//     #[error("Invalid assets requested for withdrawal.")]
//     InvalidWithdrawalAssets {},

//     #[error("Invalid vault token deposited.")]
//     InvalidVaultTokenDeposited {},

//     #[error("Invalid base token.")]
//     InvalidBaseToken {},

//     #[error("asset field does not equal coins sent")]
//     InvalidAssetField {},

//     #[error("Invalid reward liquidation target. Reward liquidation target must be one of the pool assets. Expected one of: {expected:?}. Got {actual:?}")]
//     InvalidRewardLiquidationTarget {
//         expected: Vec<AssetInfo>,
//         actual: AssetInfo,
//     },

//     #[error("Unknown reply ID: {0}")]
//     UnknownReplyId(u64),

//     #[error("No data in SubMsgResponse")]
//     NoDataInSubMsgResponse {},

//     #[error("{0}")]
//     Generic(String),
// }

// impl From<String> for ContractError {
//     fn from(val: String) -> Self {
//         ContractError::Generic(val)
//     }
// }

// impl From<&str> for ContractError {
//     fn from(val: &str) -> Self {
//         ContractError::Generic(val.into())
//     }
// }

// impl From<ContractError> for StdError {
//     fn from(e: ContractError) -> Self {
//         StdError::generic_err(e.to_string())
//     }
// }

use cosmwasm_std::{
    CheckedFromRatioError, CheckedMultiplyRatioError, Coin, ConversionOverflowError, Decimal256,
    DivideByZeroError, OverflowError, StdError, Uint128,
};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Position Not Found")]
    PositionNotFound {},

    #[error("{0}")]
    DivideByZeroError(#[from] DivideByZeroError),

    #[error("{0}")]
    CheckedMultiplyRatioError(#[from] CheckedMultiplyRatioError),

    #[error("Overflow")]
    Overflow {},

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("{0}")]
    ConversionOverflowError(#[from] ConversionOverflowError),

    #[error("{0}")]
    MultiplyRatioError(#[from] CheckedFromRatioError),

    #[error("Price must be between 0.000000000001 and 100000000000000000000000000000000000000. Got {:?}", price)]
    PriceBoundError { price: Decimal256 },

    #[error("{0}")]
    PaymentError(#[from] cw_utils::PaymentError),

    #[error(
        "Deposit amount missmatch. Expected: {:?}, Received: {:?}",
        expected,
        received
    )]
    DepositMismatch {
        expected: Uint128,
        received: Uint128,
    },

    #[error(
        "Slippage tolerance must be a number between 0 and 10000. Got {}",
        slippage_tolerance
    )]
    InvalidSlippageTolerance { slippage_tolerance: Uint128 },

    #[error("This message does no accept funds")]
    NonPayable {},

    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.

    // todo: add apollo errors one by one and see what gives us type errors
    // apollo errors below (remove from above when you add)
    #[error("Unexpected funds sent. Expected: {expected:?}, Actual: {actual:?}")]
    UnexpectedFunds {
        expected: Vec<Coin>,
        actual: Vec<Coin>,
    },
}
