use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Base(#[from] cw20_base::ContractError),

    #[error("{0}")]
    PaymentError(#[from] cw_utils::PaymentError),

    #[error("{0}")]
    QueueError(String),

    #[error("not enough funds in the strategy to withdraw")]
    InsufficientOutStandingFunds,

    #[error("{}")]
    AmountOverflow{}
}
