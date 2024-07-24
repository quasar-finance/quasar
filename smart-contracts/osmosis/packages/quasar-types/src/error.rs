use cosmwasm_std::{
    CheckedFromRatioError, Coin, DivideByZeroError, IbcOrder, OverflowError, StdError,
};
use prost::DecodeError;
use thiserror::Error;

use crate::ica::handshake::{Encoding, TxType, Version};

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("No Counterparty Ica Address")]
    NoCounterpartyIcaAddress,

    #[error("Could not deserialize counterparty ica metadata, got {raw_metadata}, error: {error}")]
    InvalidCounterpartyIcaMetadata { raw_metadata: String, error: String },

    #[error("Could not deserialize ica metadata, got {raw_metadata}, error: {error}")]
    InvalidIcaMetadata { raw_metadata: String, error: String },

    #[error("Incorrect ICA version, got {version}, want {contract_version}")]
    InvalidIcaVersion {
        version: Version,
        contract_version: Version,
    },

    #[error("Incorrect ICA version, got {encoding}, want {contract_encoding}")]
    InvalidIcaEncoding {
        encoding: Encoding,
        contract_encoding: Encoding,
    },

    #[error("Incorrect ICA version, got {tx_type}, want {contract_tx_type}")]
    InvalidIcaTxType {
        tx_type: TxType,
        contract_tx_type: TxType,
    },

    #[error("Incorrect IbcOrder")]
    IncorrectIbcOrder { expected: IbcOrder, got: IbcOrder },

    #[error("invalid Ibc version")]
    InvalidIbcVersion { version: String },

    #[error("invalid type url, expected {expected}  and found {actual}")]
    UnpackInvalidTypeUrl { expected: String, actual: String },

    #[error("{0}")]
    DecodeError(#[from] DecodeError),

    #[error("found empty coin ratio")]
    EmptyCoinRatio,

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("{0}")]
    DivideByZeroError(#[from] DivideByZeroError),
    #[error("{0}")]
    CheckedFromRatioError(#[from] CheckedFromRatioError),
}

#[derive(Error, Debug, PartialEq)]
pub enum FundsError {
    #[error("Only {0} deposit asset(s) supported.")]
    InvalidAssets(usize),

    #[error("Wrong denom, expected {0}.")]
    WrongDenom(String),
}

pub fn assert_fund_length(length: usize, expected_length: usize) -> Result<(), FundsError> {
    if length != expected_length {
        return Err(FundsError::InvalidAssets(expected_length));
    }
    Ok(())
}

pub fn assert_denom(denom: &str, expected_denom: &str) -> Result<(), FundsError> {
    if denom != expected_denom {
        return Err(FundsError::WrongDenom(expected_denom.into()));
    }
    Ok(())
}

pub fn assert_funds_single_token(funds: &[Coin], expected_denom: &str) -> Result<(), FundsError> {
    assert_fund_length(funds.len(), 1)?;
    assert_denom(&funds[0].denom, expected_denom)
}
