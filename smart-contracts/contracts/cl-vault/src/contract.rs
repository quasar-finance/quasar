use crate::error::{ContractError, ContractResult};
use crate::instantiate::{
    handle_create_denom_reply, handle_instantiate, handle_instantiate_create_position_reply,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, ModifyRangeMsg, QueryMsg};
use crate::query::{
    query_assets_from_shares, query_dex_router, query_info, query_metadata, query_pool,
    query_position, query_total_assets, query_total_vault_token_supply, query_user_assets,
    query_user_balance, query_user_rewards, query_verify_tick_cache, RangeAdminResponse,
};
use crate::reply::Replies;
use crate::rewards::{
    execute_collect_rewards, execute_distribute_rewards, handle_collect_incentives_reply,
    handle_collect_spread_rewards_reply,
};

use crate::state::{ModifyRangeState, DEX_ROUTER, MODIFY_RANGE_STATE, RANGE_ADMIN};
use crate::vault::admin::execute_admin;
use crate::vault::claim::execute_claim_user_rewards;
use crate::vault::deposit::{execute_exact_deposit, handle_deposit_create_position_reply};
use crate::vault::merge::{
    execute_merge, handle_merge_create_position_reply, handle_merge_withdraw_reply,
};
use crate::vault::range::{
    execute_update_range, get_range_admin, handle_initial_create_position_reply,
    handle_iteration_create_position_reply, handle_merge_response, handle_swap_reply,
    handle_withdraw_position_reply,
};
use crate::vault::withdraw::{execute_withdraw, handle_withdraw_user_reply};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Reply, Response,
};
use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cl-vault";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    handle_instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        cw_vault_multi_standard::VaultStandardExecuteMsg::AnyDeposit {
            amount: _,
            asset: _,
            recipient: _,
        } => unimplemented!(),
        cw_vault_multi_standard::VaultStandardExecuteMsg::ExactDeposit { recipient } => {
            execute_exact_deposit(deps, env, info, recipient)
        }
        cw_vault_multi_standard::VaultStandardExecuteMsg::Redeem { recipient, amount } => {
            execute_withdraw(deps, env, info, recipient, amount.into())
        }
        cw_vault_multi_standard::VaultStandardExecuteMsg::VaultExtension(vault_msg) => {
            match vault_msg {
                crate::msg::ExtensionExecuteMsg::Admin(admin_msg) => {
                    execute_admin(deps, info, admin_msg)
                }
                crate::msg::ExtensionExecuteMsg::Merge(msg) => execute_merge(deps, env, info, msg),
                crate::msg::ExtensionExecuteMsg::ModifyRange(ModifyRangeMsg {
                    lower_price,
                    upper_price,
                    max_slippage,
                    ratio_of_swappable_funds_to_use,
                    twap_window_seconds,
                    recommended_swap_route,
                    force_swap_route,
                }) => execute_update_range(
                    deps,
                    env,
                    info,
                    lower_price,
                    upper_price,
                    max_slippage,
                    ratio_of_swappable_funds_to_use,
                    twap_window_seconds,
                    recommended_swap_route,
                    force_swap_route,
                ),
                crate::msg::ExtensionExecuteMsg::CollectRewards { amount_of_users } => {
                    execute_collect_rewards(deps, env, amount_of_users)
                }
                crate::msg::ExtensionExecuteMsg::DistributeRewards { amount_of_users } => {
                    execute_distribute_rewards(deps, env, amount_of_users)
                }
                crate::msg::ExtensionExecuteMsg::ClaimRewards {} => {
                    execute_claim_user_rewards(deps, info.sender.as_str())
                }
            }
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> ContractResult<Binary> {
    match msg {
        cw_vault_multi_standard::VaultStandardQueryMsg::VaultStandardInfo {} => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::Info {} => {
            Ok(to_json_binary(&query_info(deps)?)?)
        }
        cw_vault_multi_standard::VaultStandardQueryMsg::PreviewDeposit { assets: _ } => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::DepositRatio => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::PreviewRedeem { amount: shares } => Ok(
            to_json_binary(&query_assets_from_shares(deps, env, shares)?)?,
        ),
        cw_vault_multi_standard::VaultStandardQueryMsg::TotalAssets {} => {
            Ok(to_json_binary(&query_total_assets(deps, env)?)?)
        }
        cw_vault_multi_standard::VaultStandardQueryMsg::TotalVaultTokenSupply {} => {
            Ok(to_json_binary(&query_total_vault_token_supply(deps)?)?)
        }
        cw_vault_multi_standard::VaultStandardQueryMsg::ConvertToShares { amount: _ } => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::ConvertToAssets { amount: shares } => Ok(
            to_json_binary(&query_assets_from_shares(deps, env, shares)?)?,
        ),
        cw_vault_multi_standard::VaultStandardQueryMsg::VaultExtension(msg) => match msg {
            crate::msg::ExtensionQueryMsg::Metadata {} => {
                Ok(to_json_binary(&query_metadata(deps)?)?)
            }
            crate::msg::ExtensionQueryMsg::DexRouter {} => {
                Ok(to_json_binary(&query_dex_router(deps)?)?)
            }
            crate::msg::ExtensionQueryMsg::Balances(msg) => match msg {
                crate::msg::UserBalanceQueryMsg::UserSharesBalance { user } => {
                    Ok(to_json_binary(&query_user_balance(deps, user)?)?)
                }
                crate::msg::UserBalanceQueryMsg::UserRewards { user } => {
                    Ok(to_json_binary(&query_user_rewards(deps, user)?)?)
                }
                crate::msg::UserBalanceQueryMsg::UserAssetsBalance { user } => {
                    Ok(to_json_binary(&query_user_assets(deps, env, user)?)?)
                }
            },
            crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(msg) => match msg {
                crate::msg::ClQueryMsg::Pool {} => Ok(to_json_binary(&query_pool(deps)?)?),
                crate::msg::ClQueryMsg::Position {} => Ok(to_json_binary(&query_position(deps)?)?),
                crate::msg::ClQueryMsg::RangeAdmin {} => {
                    let range_admin = get_range_admin(deps)?;
                    Ok(to_json_binary(&RangeAdminResponse {
                        address: range_admin.to_string(),
                    })?)
                }
                crate::msg::ClQueryMsg::VerifyTickCache => {
                    Ok(to_json_binary(&query_verify_tick_cache(deps)?)?)
                }
            },
        },
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    match msg.id.into() {
        Replies::InstantiateCreatePosition => {
            handle_instantiate_create_position_reply(deps, env, msg.result)
        }
        Replies::DepositCreatePosition => {
            handle_deposit_create_position_reply(deps, env, msg.result)
        }
        Replies::CollectIncentives => handle_collect_incentives_reply(deps, env, msg.result),
        Replies::CollectSpreadRewards => handle_collect_spread_rewards_reply(deps, env, msg.result),
        Replies::WithdrawPosition => handle_withdraw_position_reply(deps, env, msg.result),
        Replies::RangeInitialCreatePosition => {
            handle_initial_create_position_reply(deps, env, msg.result)
        }
        Replies::RangeIterationCreatePosition => {
            handle_iteration_create_position_reply(deps, env, msg.result)
        }
        Replies::Swap => handle_swap_reply(deps, env, msg.result),
        Replies::Merge => handle_merge_response(deps, msg.result),
        Replies::CreateDenom => handle_create_denom_reply(deps, msg.result),
        Replies::WithdrawUser => handle_withdraw_user_reply(deps, msg.result),
        Replies::WithdrawMerge => handle_merge_withdraw_reply(deps, env, msg.result),
        Replies::CreatePositionMerge => handle_merge_create_position_reply(deps, env, msg.result),
        Replies::Unknown => unimplemented!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    let new_range_admin_addr = deps.api.addr_validate(&msg.range_admin)?;
    RANGE_ADMIN.save(deps.storage, &new_range_admin_addr)?;

    let new_router_addr = deps.api.addr_validate(&msg.dex_router)?;
    DEX_ROUTER.save(deps.storage, &new_router_addr)?;

    let def_state = ModifyRangeState {
        lower_tick: 0,
        upper_tick: 0,
        max_slippage: Decimal::zero(),
        new_range_position_ids: vec![],
        ratio_of_swappable_funds_to_use: Decimal::zero(),
        twap_window_seconds: 0,
        recommended_swap_route: None,
        force_swap_route: Some(false),
    };

    let mut modify_range_state = MODIFY_RANGE_STATE
        .load(deps.storage)
        .unwrap_or(Some(def_state));

    if let Some(ref mut state) = modify_range_state {
        state.recommended_swap_route = None;
        state.force_swap_route = Some(false);
    }

    MODIFY_RANGE_STATE.save(deps.storage, &modify_range_state)?;

    Ok(Response::new()
        .add_attribute("method", "migrate")
        .add_attribute("message", "migrated successfully"))
}
