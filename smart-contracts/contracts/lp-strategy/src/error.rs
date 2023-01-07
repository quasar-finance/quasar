use std::num::ParseIntError;
use std::fmt::Debug;
use cosmwasm_std::{StdError, Uint128, Addr, OverflowError, DivideByZeroError};
use quasar_types::error::Error as QError;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use thiserror::Error;

use crate::helpers::IbcMsgKind;

/// Never is a placeholder to ensure we don't return any errors
#[derive(Error, Debug)]
pub enum Never {}

type TrapResult<T> = Result<T, ContractError>;

trait Trappable<T> {
    fn trap(self) -> T;
}

impl<T> Trappable<T> for TrapResult<T> {
    fn trap(self) -> T {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Trap {
    // A string describing the trapped error
    error: String,
    // the failed step
    step: IbcMsgKind,
    // the address of the user whose calls failed
    addres: Addr,
    // the amount of funds of the claim that faild
    amount: Uint128,
}

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

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("{0}")]
    DivideByZeroError(#[from] DivideByZeroError),
}
