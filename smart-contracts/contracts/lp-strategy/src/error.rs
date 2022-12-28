use std::num::ParseIntError;

use cosmwasm_std::StdError;
use quasar_types::error::Error as QError;
use thiserror::Error;

/// Never is a placeholder to ensure we don't return any errors
#[derive(Error, Debug)]
pub enum Never {}

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Base(#[from] cw20_base::ContractError),

    #[error("{0}")]
    QError(#[from] QError),

    #[error("{0}")]
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("{0}")]
    QueueError(String),

    #[error("no counterpart ica address found")]
    NoCounterpartyIcaAddress,

    #[error("channel is not an ica channel")]
    NoIcaChannel,

    #[error("Could not deserialize ack: {err}, payload was {b64_bin}")]
    DeserializeIcaAck { b64_bin: String, err: String },

    #[error("not enough funds in the strategy to withdraw")]
    InsufficientOutStandingFunds,

    #[error("{0}")]
    DecodeError(#[from] prost::DecodeError),

    #[error("{0}")]
    ParseIntError(#[from] ParseIntError),
}
