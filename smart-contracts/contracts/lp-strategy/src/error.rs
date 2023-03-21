use cosmwasm_std::{CheckedMultiplyRatioError, DivideByZeroError, OverflowError, StdError};
use quasar_types::error::Error as QError;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::num::ParseIntError;

use crate::helpers::IbcMsgKind;
use std::str::Utf8Error;
use thiserror::Error;

/// Never is a placeholder to ensure we don't return any errors
#[derive(Error, Debug)]
pub enum Never {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Trap {
    // A string describing the trapped error
    pub error: String,
    // the failed step and underlying values
    pub step: IbcMsgKind,
    // last_succesful notes whether the IbcMsg of step was succesful on the counterparty chain
    pub last_succesful: bool,
}

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Base(#[from] cw20_base::ContractError),

    #[error("{0}")]
    QError(#[from] QError),

    #[error("map has duplicate key while no key should be present")]
    DuplicateKey,

    #[error("caller is unauthorized")]
    Unauthorized,

    #[error("{0}")]
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("not enough claims")]
    InsufficientClaims,

    #[error("not enough claims")]
    InsufficientFunds,

    #[error("base denom not found")]
    BaseDenomNotFound,

    #[error("quote denom not found")]
    QuoteDenomNotFound,

    #[error("No item in the queue {} while an item was expected", queue)]
    QueueItemNotFound { queue: String },

    #[error("no counterpart ica address found")]
    NoCounterpartyIcaAddress,

    #[error("ica channel is already set while it should be unset")]
    IcaChannelAlreadySet,

    #[error("channel is not an ica channel")]
    NoIcaChannel,

    #[error("channel is not an icq channel")]
    NoIcqChannel,

    #[error("no connection is found")]
    NoConnectionFound,

    #[error("incorrect connection id")]
    IncorrectConnection,

    #[error("raw ack in recovery could not be handled")]
    IncorrectRecoveryAck,

    #[error("no timestamp time found for ibc packets")]
    NoTimestampTime,

    #[error("reply data not found")]
    NoReplyData,

    #[error("Could not deserialize ack: {err}, payload was {b64_bin}")]
    DeserializeIcaAck { b64_bin: String, err: String },

    #[error("Could not find returning transfer")]
    ReturningTransferNotFound,

    #[error("amount of returning transfer is not the same as the expected amount")]
    ReturningTransferIncorrectAmount,

    #[error("Shares are still unbonding")]
    SharesNotYetUnbonded,

    #[error("found incorrect raw amount type")]
    IncorrectRawAmount,

    #[error("{0}")]
    DecodeError(#[from] prost::DecodeError),

    #[error("parse int error: {error} caused by {value}")]
    ParseIntError { error: ParseIntError, value: String },

    #[error("parse int error: {error} caused by {value}")]
    ParseDecError { error: StdError, value: String },

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("{0}")]
    DivideByZeroError(#[from] DivideByZeroError),

    #[error("{0}")]
    CheckedMultiplyRatioError(#[from] CheckedMultiplyRatioError),

    #[error("{0}")]
    Utf8Error(#[from] Utf8Error),

    #[error("{0}")]
    SerdeJsonDe(#[from] serde_json_wasm::de::Error),

    #[error("could not serialize to json")]
    SerdeJsonSer,

    #[error("The Callback has no amount set")]
    CallbackHasNoAmount,
}
