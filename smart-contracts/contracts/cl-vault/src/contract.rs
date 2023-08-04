#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::CosmosMsg;
use cosmwasm_std::Reply;
use cosmwasm_std::SubMsg;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::Pool;
use osmosis_std::types::osmosis::poolmanager::v1beta1::PoolmanagerQuerier;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenomResponse;

use crate::error::ContractError;
use crate::error::ContractResult;
use crate::msg::ModifyRangeMsg;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::query_info;
use crate::query::query_pool;
use crate::reply::handle_reply;
use crate::reply::Replies;
use crate::state::LOCKUP_DURATION;
use crate::state::VAULT_DENOM;
use crate::state::{PoolConfig, POOL_CONFIG, VAULT_CONFIG};
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
    env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    VAULT_CONFIG.save(deps.storage, &msg.config)?;

    let pool: Pool = PoolmanagerQuerier::new(&deps.querier)
        .pool(msg.pool_id)?
        .pool
        .ok_or(ContractError::PoolNotFound {
            pool_id: msg.pool_id,
        })?
        .try_into()
        .unwrap();

    POOL_CONFIG.save(
        deps.storage,
        &PoolConfig {
            pool_id: pool.id,
            token0: pool.token0,
            token1: pool.token1,
        },
    )?;

    let admin = deps.api.addr_validate(&msg.admin)?;

    ADMIN_ADDRESS.save(deps.storage, &admin)?;
    RANGE_ADMIN.save(deps.storage, &deps.api.addr_validate(&msg.range_admin)?)?;

    LOCKUP_DURATION.save(deps.storage, &cw_utils::Duration::Time(msg.lockup_duration))?;

    let create_denom: CosmosMsg = MsgCreateDenom {
        sender: env.contract.address.to_string(),
        subdenom: msg.vault_token_subdenom,
    }
    .into();

    Ok(Response::new().add_submessage(SubMsg::reply_on_success(
        create_denom,
        Replies::CreateDenom as u64,
    )))
}

pub fn handle_create_denom_reply(deps: DepsMut, data: Binary) -> Result<Response, ContractError> {
    let response: MsgCreateDenomResponse = data.try_into()?;
    VAULT_DENOM.save(deps.storage, &response.new_token_denom)?;

    Ok(Response::new().add_attribute("vault_denom", response.new_token_denom))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        cw_vault_multi_standard::VaultStandardExecuteMsg::SingleDeposit {
            amount,
            asset,
            recipient,
        } => todo!(),
        cw_vault_multi_standard::VaultStandardExecuteMsg::MultiDeposit { recipient } => todo!(),
        cw_vault_multi_standard::VaultStandardExecuteMsg::Redeem { recipient, amount } => todo!(),
        cw_vault_multi_standard::VaultStandardExecuteMsg::VaultExtension(vault_msg) => {
            match vault_msg {
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
            }
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> ContractResult<Binary> {
    match msg {
        cw_vault_multi_standard::VaultStandardQueryMsg::VaultStandardInfo {} => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::Info {} => query_info(deps),
        cw_vault_multi_standard::VaultStandardQueryMsg::PreviewDeposit { assets } => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::DepositRatio => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::PreviewRedeem { amount } => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::TotalAssets {} => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::TotalVaultTokenSupply {} => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::ConvertToShares { amount } => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::ConvertToAssets { amount } => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::VaultExtension(msg) => match msg {
            crate::msg::ExtensionQueryMsg::Lockup(_) => todo!(),
            crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(msg) => match msg {
                crate::msg::ClQueryMsg::Pool {} => query_pool(deps),
            },
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    handle_reply(deps, env, msg)
}

#[cfg(test)]
mod tests {}
