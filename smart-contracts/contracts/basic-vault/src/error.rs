use cosmwasm_std::{CheckedMultiplyRatioError, OverflowError, StdError, Uint128};
use quasar_types::error::Error as QError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Validator '{validator}' not in current validator set")]
    NotInValidatorSet { validator: String },

    #[error("Different denominations in bonds: '{denom1}' vs. '{denom2}'")]
    DifferentBondDenom { denom1: String, denom2: String },

    #[error("Stored bonded {stored}, but query bonded {queried}")]
    BondedMismatch { stored: Uint128, queried: Uint128 },

    #[error("Incorrect bonding ratio")]
    IncorrectBondingRatio {},

    #[error("No {denom} tokens sent")]
    EmptyBalance { denom: String },

    #[error("Must unbond at least {min_bonded}")]
    UnbondTooSmall { min_bonded: Uint128 },

    #[error("Cannot withdraw without vault tokens")]
    NoFunds {},

    #[error("Insufficient balance in contract to process claim")]
    BalanceTooSmall {},

    #[error("No claims that can be released currently")]
    NothingToClaim {},

    #[error("Cannot set to own account")]
    CannotSetOwnAccount {},

    #[error("Invalid expiration")]
    InvalidExpiration {},

    #[error("Invalid zero amount")]
    InvalidZeroAmount {},

    #[error("Allowance is expired")]
    Expired {},

    #[error("No allowance for this account")]
    NoAllowance {},

    #[error("Minting cannot exceed the cap")]
    CannotExceedCap {},

    #[error("Duplicate initial balance addresses")]
    DuplicateInitialBalanceAddresses {},

    #[error("Incorrect callback id, expected: {expected}, got: {:?}", ids)]
    IncorrectCallbackId { expected: String, ids: Vec<String> },

    #[error("Overflow error: {0}")]
    OverflowError(String),

    #[error("Multiply ratio error: {0}")]
    MultiplyRatioError(String),

    #[error("Missing bond response")]
    MissingBondResponse {},

    #[error("Token weight vector is empty")]
    TokenWeightsIsEMpty {},

    #[error("Coins vector is empty")]
    CoinsVectorIsEmpty {},

    #[error("Denom not found in coins vector")]
    DenomNotFoundInCoinsVector {},

    #[error("User does not have pending unbonds")]
    UserDoNotHavePendingUnbonds {},

    #[error("Bond response is empty")]
    BondResponseIsEmpty {},

    #[error("Unbond is empty")]
    UnbondIsEmpty {},

    #[error("Unbond stub is empty")]
    UnbondStubIsEmpty {},

    #[error("Coins weight vector is empty")]
    CoinsWeightVectorIsEmpty {},

    #[error("{0}")]
    QError(#[from] QError),
}

impl From<OverflowError> for ContractError {
    fn from(err: OverflowError) -> Self {
        ContractError::OverflowError(format!("{err}"))
    }
}

impl From<CheckedMultiplyRatioError> for ContractError {
    fn from(err: CheckedMultiplyRatioError) -> Self {
        ContractError::OverflowError(format!("{err}"))
    }
}

impl From<cw_utils::PaymentError> for ContractError {
    fn from(err: cw_utils::PaymentError) -> Self {
        ContractError::Std(StdError::generic_err(err.to_string()))
    }
}

impl From<cw20_base::ContractError> for ContractError {
    fn from(err: cw20_base::ContractError) -> Self {
        match err {
            cw20_base::ContractError::Std(error) => ContractError::Std(error),
            cw20_base::ContractError::Unauthorized {} => ContractError::Unauthorized {},
            cw20_base::ContractError::CannotSetOwnAccount {} => {
                ContractError::CannotSetOwnAccount {}
            }
            cw20_base::ContractError::InvalidExpiration {} => ContractError::InvalidExpiration {},
            cw20_base::ContractError::InvalidZeroAmount {} => ContractError::InvalidZeroAmount {},
            cw20_base::ContractError::Expired {} => ContractError::Expired {},
            cw20_base::ContractError::NoAllowance {} => ContractError::NoAllowance {},
            cw20_base::ContractError::CannotExceedCap {} => ContractError::CannotExceedCap {},
            // This should never happen, as this contract doesn't use logo
            cw20_base::ContractError::LogoTooBig {}
            | cw20_base::ContractError::InvalidPngHeader {}
            | cw20_base::ContractError::InvalidXmlPreamble {} => {
                ContractError::Std(StdError::generic_err(err.to_string()))
            }
            cw20_base::ContractError::DuplicateInitialBalanceAddresses {} => {
                ContractError::DuplicateInitialBalanceAddresses {}
            }
        }
    }
}
