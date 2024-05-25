use cosmwasm_std::{OverflowError, StdError};
use cw_controllers::AdminError;
use cw_utils::ParseReplyError;
use thiserror::Error;

// pub type ContractResult<T> = Result<T, ContractError>;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Admin(#[from] AdminError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("{0}")]
    ParseFailure(#[from] ParseReplyError),

    #[error("Unknown reply")]
    UnknownReply,

    #[error("Gauge don't exists: {addr}")]
    NoSuchGauge{ addr: String },

    #[error("Start time must be ahead")]
    StartTimeMustBeAhead,

    #[error("End time must be ahead")]
    EndTimeMustBeAhead,

    #[error("End time must be ahead")]
    EndTimeBiggerThanStart,

    #[error("Parsing previous version error")]
    ParsingPrevVersion,

    #[error("Parsing new version error")]
    ParsingNewVersion,

    #[error("Msg version is not equal contract new version")]
    ImproperMsgVersion,

    #[error("Unauthorized")]
    Unauthorized {},
}
