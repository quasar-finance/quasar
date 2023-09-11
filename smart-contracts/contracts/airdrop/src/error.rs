use cosmwasm_std::StdError;
use cw_asset::AssetError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum AirdropErrors {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Asset(#[from] AssetError),

    #[error("Unauthorized")]
    Unauthorized {},
}
