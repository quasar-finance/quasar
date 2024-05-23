use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;

// pub type ContractResult<T> = Result<T, ContractError>;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("Parsing previous version error")]
    ParsingPrevVersion,

    #[error("Parsing new version error")]
    ParsingNewVersion,

    #[error("Msg version is not equal contract new version")]
    ImproperMsgVersion,

    #[error("Unauthorized")]
    Unauthorized {},
}
