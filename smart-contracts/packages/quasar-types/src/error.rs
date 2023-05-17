use std::string::FromUtf8Error;

use cosmwasm_std::{CheckedFromRatioError, DivideByZeroError, IbcOrder, OverflowError, StdError};
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

    #[error("{0}")]
    InvalidUTF8(#[from] FromUtf8Error),

    #[error("Item {} is empty", item)]
    ItemIsEmpty { item: String },

    #[error("Key {:?} is not present in map {}", key, map)]
    KeyNotPresentInMap { key: String, map: String },
}
