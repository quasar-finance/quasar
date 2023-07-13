#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cl-vault";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    todo!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        cw_vault_standard::VaultStandardExecuteMsg::Deposit { amount, recipient } => todo!(),
        cw_vault_standard::VaultStandardExecuteMsg::Redeem { recipient, amount } => todo!(),
        cw_vault_standard::VaultStandardExecuteMsg::VaultExtension(_) => todo!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        cw_vault_standard::VaultStandardQueryMsg::VaultStandardInfo {} => todo!(),
        cw_vault_standard::VaultStandardQueryMsg::Info {} => todo!(),
        cw_vault_standard::VaultStandardQueryMsg::PreviewDeposit { amount } => todo!(),
        cw_vault_standard::VaultStandardQueryMsg::PreviewRedeem { amount } => todo!(),
        cw_vault_standard::VaultStandardQueryMsg::TotalAssets {} => todo!(),
        cw_vault_standard::VaultStandardQueryMsg::TotalVaultTokenSupply {} => todo!(),
        cw_vault_standard::VaultStandardQueryMsg::ConvertToShares { amount } => todo!(),
        cw_vault_standard::VaultStandardQueryMsg::ConvertToAssets { amount } => todo!(),
        cw_vault_standard::VaultStandardQueryMsg::VaultExtension(_) => todo!(),
    }
}

#[cfg(test)]
mod tests {}
