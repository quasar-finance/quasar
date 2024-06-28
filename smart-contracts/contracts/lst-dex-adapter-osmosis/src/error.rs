use abstract_app::objects::ans_host::AnsHostError;
use abstract_app::sdk::AbstractSdkError;
use abstract_app::std::AbstractError;
use abstract_app::AppError;
use cosmwasm_std::StdError;
use cw_asset::AssetError;
use quasar_types::error::FundsError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DexAdapterError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Abstract(#[from] AbstractError),

    #[error("{0}")]
    AbstractSdk(#[from] AbstractSdkError),

    #[error("{0}")]
    AnsHost(#[from] AnsHostError),

    #[error("{0}")]
    Asset(#[from] AssetError),

    #[error("{0}")]
    DappError(#[from] AppError),

    #[error("Price is not low enough.")]
    InvalidPrice {},

    #[error("{0}")]
    Funds(#[from] FundsError),
}
