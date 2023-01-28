use cosmwasm_std::{Addr, DivideByZeroError, OverflowError, StdError, Uint128};
use quasar_types::error::Error as QError;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::num::ParseIntError;

use std::str::Utf8Error;
use thiserror::Error;

use crate::helpers::IbcMsgKind;
use crate::state::OngoingDeposit;

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

    #[error("{0}")]
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("not enough claims")]
    InsufficientClaims,

    #[error("base denom not found")]
    BaseDenomNotFound,

    #[error("quote denom not found")]
    QuoteDenomNotFound,

    #[error("No item in the queue while an item was expected")]
    QueueItemNotFound,

    #[error("no counterpart ica address found")]
    NoCounterpartyIcaAddress,

    #[error("ica channel is already set while it should be unset")]
    IcaChannelAlreadySet,

    #[error("channel is not an ica channel")]
    NoIcaChannel,

    #[error("channel is not an icq channel")]
    NoIcqChannel,

    #[error("no timestamp time found for ibc packets")]
    NoTimestampTime,

    #[error("reply data not found")]
    NoReplyData,

    #[error("Could not deserialize ack: {err}, payload was {b64_bin}")]
    DeserializeIcaAck { b64_bin: String, err: String },

    #[error("Shares are still unbonding")]
    SharesNotYetUnbonded,

    #[error("{0}")]
    DecodeError(#[from] prost::DecodeError),

    #[error("{0}")]
    ParseIntError(#[from] ParseIntError),

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("{0}")]
    DivideByZeroError(#[from] DivideByZeroError),

    #[error("{0}")]
    Utf8Error(#[from] Utf8Error),
}
