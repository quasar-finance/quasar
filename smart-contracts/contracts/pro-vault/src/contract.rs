#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response,
};

use crate::error::{ContractError, ContractResult};
use crate::instantiate::handle_instantiate;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::reply::Replies;


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    todo!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        cw_vault_standard::VaultStandardExecuteMsg::Deposit { amount, recipient } => todo!(),
        cw_vault_standard::VaultStandardExecuteMsg::Redeem { recipient, amount } => todo!(),
        cw_vault_standard::VaultStandardExecuteMsg::VaultExtension(_) => todo!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> ContractResult<Binary> {
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id.into() {
        Replies::Unknown => unimplemented!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    todo!()
}
