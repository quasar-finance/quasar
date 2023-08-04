#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, ModifyRangeMsg, QueryMsg};

use crate::state::{ADMIN_ADDRESS, RANGE_ADMIN};
use crate::vault::admin::execute_admin;
use crate::vault::deposit::execute_deposit;
use crate::vault::range::execute_modify_range;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cl-vault";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // save contract admin
    ADMIN_ADDRESS.save(deps.storage, &deps.api.addr_validate(&msg.admin)?)?;
    // save range admin
    RANGE_ADMIN.save(deps.storage, &deps.api.addr_validate(&msg.range_admin)?)?;
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
        cw_vault_standard::VaultStandardExecuteMsg::Redeem {
            recipient: _,
            amount: _,
        } => todo!(),
        cw_vault_standard::VaultStandardExecuteMsg::VaultExtension(vault_msg) => match vault_msg {
            crate::msg::ExtensionExecuteMsg::Callback(callback_msg) => match callback_msg {
                crate::msg::CallbackMsg::SellRewards {} => todo!(),
                crate::msg::CallbackMsg::ProvideLiquidity {} => todo!(),
                crate::msg::CallbackMsg::Stake {
                    base_token_balance_before: _,
                } => todo!(),
                crate::msg::CallbackMsg::MintVaultToken {
                    amount: _,
                    recipient: _,
                } => todo!(),
            },
            crate::msg::ExtensionExecuteMsg::Admin(admin_msg) => {
                execute_admin(deps, info, admin_msg)
            }
            crate::msg::ExtensionExecuteMsg::Lockup(_) => todo!(),
            crate::msg::ExtensionExecuteMsg::ModifyRange(ModifyRangeMsg {
                lower_price,
                upper_price,
            }) => execute_modify_range(deps, env, info, lower_price, upper_price),
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        cw_vault_standard::VaultStandardQueryMsg::VaultStandardInfo {} => todo!(),
        cw_vault_standard::VaultStandardQueryMsg::Info {} => todo!(),
        cw_vault_standard::VaultStandardQueryMsg::PreviewDeposit { amount: _ } => todo!(),
        cw_vault_standard::VaultStandardQueryMsg::PreviewRedeem { amount: _ } => todo!(),
        cw_vault_standard::VaultStandardQueryMsg::TotalAssets {} => todo!(),
        cw_vault_standard::VaultStandardQueryMsg::TotalVaultTokenSupply {} => todo!(),
        cw_vault_standard::VaultStandardQueryMsg::ConvertToShares { amount: _ } => todo!(),
        cw_vault_standard::VaultStandardQueryMsg::ConvertToAssets { amount: _ } => todo!(),
        cw_vault_standard::VaultStandardQueryMsg::VaultExtension(_) => todo!(),
    }
}

#[cfg(test)]
mod tests {}
