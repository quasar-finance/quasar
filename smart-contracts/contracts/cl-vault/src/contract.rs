#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::swap;
use crate::vault::admin::execute_admin;
use crate::vault::deposit::execute_deposit;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cl-vault";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        cw_vault_standard::VaultStandardExecuteMsg::Deposit { amount, recipient } => {
            execute_deposit(deps, env, &info, amount, recipient)
        }
        cw_vault_standard::VaultStandardExecuteMsg::Redeem { recipient, amount } => todo!(),
        cw_vault_standard::VaultStandardExecuteMsg::VaultExtension(vault_msg) => match vault_msg {
            crate::msg::ExtensionExecuteMsg::Callback(callback_msg) => match callback_msg {
                crate::msg::CallbackMsg::SellRewards {} => todo!(),
                crate::msg::CallbackMsg::ProvideLiquidity {} => todo!(),
                crate::msg::CallbackMsg::Stake {
                    base_token_balance_before,
                } => todo!(),
                crate::msg::CallbackMsg::MintVaultToken { amount, recipient } => todo!(),
            },
            crate::msg::ExtensionExecuteMsg::Admin(admin_msg) => {
                execute_admin(deps, info, admin_msg)
            }
        },
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
