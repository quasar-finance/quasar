use std::env;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult};
use cw2::set_contract_version;
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Investment, Replies, INVESTMENT, REPLIES};
use crate::vault::deposit::{execute_deposit, handle_create_position_reply, handle_swap_reply};
use crate::vault::admin::execute_admin;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cl-vault";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    INVESTMENT.save(
        deps.storage,
        &Investment {
            owner: info.sender,
            base_denom: msg.base_denom,
            quote_denom: msg.quote_denom,
            pool_id: msg.pool_id,
        },
    )?;
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    // Save the ibc message together with the sequence number, to be handled properly later at the ack, we can pass the ibc_kind one to one
    // TODO this needs and error check and error handling
    let reply = REPLIES.load(deps.storage, msg.id)?;
    match reply {
        Replies::Swap { user_addr, amount0 } => {
            handle_swap_reply(deps, env, user_addr, amount0, msg)
        }
        Replies::CreatePosition { user_addr } => {
            handle_create_position_reply(deps, env, user_addr, msg)
        }
    }
}

#[cfg(test)]
mod tests {}
