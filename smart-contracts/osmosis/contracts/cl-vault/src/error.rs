use crate::state::{ADMIN_ADDRESS, RANGE_ADMIN, VAULT_CONFIG};
use cosmwasm_std::{
    Addr, CheckedFromRatioError, CheckedMultiplyFractionError, CheckedMultiplyRatioError, Coin,
    CoinFromStrError, ConversionOverflowError, Decimal, Decimal256, Decimal256RangeExceeded,
    DecimalRangeExceeded, DivideByZeroError, OverflowError, StdError, Storage, Uint128,
};
use cw_utils::PaymentError;
use prost::DecodeError;
use std::num::{ParseIntError, TryFromIntError};
use thiserror::Error;

/// AutocompoundingVault errors
#[allow(missing_docs)]
#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Pool-id {pool_id} not found")]
    PoolNotFound { pool_id: u64 },

    #[error("Position Not Found")]
    PositionNotFound,

    #[error("Sent the wrong amount of denoms")]
    IncorrectAmountFunds,

    #[error("ratio_of_swappable_funds_to_use should be >0 and <=1")]
    InvalidRatioOfSwappableFundsToUse,

    #[error("Cannot do two swaps at the same time")]
    SwapInProgress,

    #[error("Swap deposit merge state item not found")]
    SwapDepositMergeStateNotFound,

    #[error("Vault shares sent in does not equal amount requested")]
    IncorrectShares,

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

    #[error("Insufficient funds for swap. Have: {balance}, Need: {needed}")]
    InsufficientFundsForSwap { balance: Uint128, needed: Uint128 },

    #[error("Insufficient funds")]
    InsufficientFunds,

    #[error("Cannot merge positions that are in different ticks")]
    DifferentTicksInMerge,

    #[error("Tick index minimum error")]
    TickIndexMinError {},

    #[error("Tick index maximum error")]
    TickIndexMaxError {},

    #[error("Price must be between 0.000000000001 and 100000000000000000000000000000000000000. Got {:?}", price)]
    PriceBoundError { price: Decimal256 },

    #[error("Cannot handle negative powers in uints")]
    CannotHandleNegativePowersInUint {},

    #[error("Invalid current tick and deposit token combination")]
    InvalidCurrentTick {},

    #[error("Tick not found in tick cache, tick: {tick}")]
    TickNotFound { tick: i64 },

    #[error("Mismatch in old and new pool tokens")]
    PoolTokenMismatch {},

    /// This function compares the address of the message sender (caller) with the current admin
    /// address stored in the state. This provides a convenient way to verify if the caller
    /// is the admin in a single line.
    #[error("Cannot force a recommended route if recommended route is passed in as None")]
    TryForceRouteWithoutRecommendedSwapRoute {},

    #[error("Swap operations for non vault funds swap cannot be empty")]
    EmptySwapOperations {},

    #[error("Migration status is closed")]
    MigrationStatusClosed {},

    #[error("Migration status is open")]
    MigrationStatusOpen {},

    #[error("Vault is not distributing rewards, claiming is needed first")]
    IsNotDistributing {},

    #[error("Vault is already distributing rewards")]
    IsDistributing {},

    #[error("Swap vault related denoms is not allowed.")]
    InvalidSwapAssets {},

    #[error("Parsing error: {msg}")]
    ParseError { msg: String },

    #[error("Missing position.")]
    MissingPosition {},

    #[error("Missing recommended swap route.")]
    MissingRecommendedSwapRoute {},

    #[error("Missing information for {asset}")]
    MissingAssetInfo { asset: String },

    #[error("Error converting {asset}: {msg}")]
    ConversionError { asset: String, msg: String },

    #[error("Position claim after period is not expired yet.")]
    ClaimAfterNotExpired {},

    // Imported errors
    #[error("{0}")]
    PaymentError(#[from] PaymentError),

    #[error("{0}")]
    ParseIntError(#[from] ParseIntError),

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("{0}")]
    ConversionOverflowError(#[from] ConversionOverflowError),

    #[error("{0}")]
    CheckedMultiplyRatio(#[from] CheckedMultiplyRatioError),

    #[error("{0}")]
    CheckedMultiplyFraction(#[from] CheckedMultiplyFractionError),

    #[error("{0}")]
    DecimalRange(#[from] DecimalRangeExceeded),

    #[error("{0}")]
    DecodeError(#[from] DecodeError),

    #[error("{0}")]
    CoinFromStrError(#[from] CoinFromStrError),

    #[error("{0}")]
    CheckedFromRatio(#[from] CheckedFromRatioError),

    #[error("{0}")]
    DivideByZeroError(#[from] DivideByZeroError),

    #[error("{0}")]
    Decimal256RangeExceededError(#[from] Decimal256RangeExceeded),

    #[error("{0}")]
    TryFromIntError(#[from] TryFromIntError),
}

pub fn assert_admin(storage: &dyn Storage, caller: &Addr) -> Result<(), ContractError> {
    if ADMIN_ADDRESS.load(storage)? != caller {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

pub fn assert_range_admin(storage: &dyn Storage, sender: &Addr) -> Result<(), ContractError> {
    let admin = RANGE_ADMIN.load(storage)?;
    if admin != sender {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

pub fn assert_swap_admin(storage: &dyn Storage, sender: &Addr) -> Result<(), ContractError> {
    let vault_config = VAULT_CONFIG.load(storage)?;
    if vault_config.swap_admin != sender {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}

pub fn assert_ratio(ratio: Decimal) -> Result<(), ContractError> {
    if ratio > Decimal::one() || ratio <= Decimal::zero() {
        return Err(ContractError::InvalidRatioOfSwappableFundsToUse {});
    }
    Ok(())
}
