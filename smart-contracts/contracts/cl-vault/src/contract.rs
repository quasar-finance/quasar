#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response,
};
use cw2::set_contract_version;
use cw_storage_plus::Item;

use crate::error::ContractError;
use crate::helpers::sort_tokens;
use crate::instantiate::{
    handle_create_denom_reply, handle_instantiate, handle_instantiate_create_position_reply,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::{
    query_assets_from_shares, query_dex_router, query_info, query_main_position, query_metadata,
    query_pool, query_positions, query_total_assets, query_total_vault_token_supply,
    query_user_assets, query_user_balance, query_verify_tick_cache, RangeAdminResponse,
};
use crate::reply::Replies;
use crate::rewards::{
    execute_collect_rewards, handle_collect_incentives_reply, handle_collect_spread_rewards_reply,
    prepend_claim_msg,
};
use crate::state::{Position, MAIN_POSITION_ID, POSITIONS};
#[allow(deprecated)]
use crate::state::{
    MigrationStatus, VaultConfig, MIGRATION_STATUS, OLD_VAULT_CONFIG, STRATEGIST_REWARDS,
    VAULT_CONFIG,
};
use crate::vault::admin::execute_admin;
use crate::vault::any_deposit::{execute_any_deposit, handle_any_deposit_swap_reply};
use crate::vault::autocompound::{
    execute_autocompound, execute_migration_step, handle_autocompound_reply,
};
use crate::vault::exact_deposit::execute_exact_deposit;
use crate::vault::merge::{
    execute_merge_position, handle_merge_create_position_reply,
    handle_merge_withdraw_position_reply,
};
use crate::vault::range::create_position::handle_range_new_create_position;
use crate::vault::range::modify_position_funds::handle_range_add_to_position_reply;
use crate::vault::range::move_position::{
    get_range_admin, handle_initial_create_position_reply, handle_iteration_create_position_reply,
    handle_merge_reply, handle_swap_reply, handle_withdraw_position_reply,
};
use crate::vault::range::update_range::execute_update_range;
use crate::vault::swap::execute_swap_non_vault_funds;
use crate::vault::withdraw::{execute_withdraw, handle_withdraw_user_reply};

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
            recipient,
            max_slippage,
        } => execute_any_deposit(deps, env, info, recipient, max_slippage),
        cw_vault_multi_standard::VaultStandardExecuteMsg::ExactDeposit { recipient } => {
            execute_exact_deposit(deps, env, info, recipient)
        }
        cw_vault_multi_standard::VaultStandardExecuteMsg::Redeem { recipient, amount } => {
            prepend_claim_msg(
                &env,
                execute_withdraw(deps, &env, info, recipient, amount.into())?,
            )
        }
        cw_vault_multi_standard::VaultStandardExecuteMsg::VaultExtension(vault_msg) => {
            match vault_msg {
                crate::msg::ExtensionExecuteMsg::Admin(admin_msg) => {
                    execute_admin(deps, info, admin_msg)
                }
                crate::msg::ExtensionExecuteMsg::Authz(msg) => match msg {
                    crate::msg::AuthzExtension::ExactDeposit {} => {
                        execute_exact_deposit(deps, env, info, None)
                    }
                    crate::msg::AuthzExtension::AnyDeposit { max_slippage } => {
                        execute_any_deposit(deps, env, info, None, max_slippage)
                    }
                    crate::msg::AuthzExtension::Redeem { amount } => prepend_claim_msg(
                        &env,
                        execute_withdraw(deps, &env, info, None, amount.into())?,
                    ),
                },
                crate::msg::ExtensionExecuteMsg::Merge(msg) => {
                    execute_merge_position(deps, env, info, msg)
                }
                crate::msg::ExtensionExecuteMsg::Autocompound {} => {
                    prepend_claim_msg(&env, execute_autocompound(deps, &env, info)?)
                }
                crate::msg::ExtensionExecuteMsg::ModifyRange(msg) => {
                    prepend_claim_msg(&env, execute_update_range(deps, &env, info, msg)?)
                }
                crate::msg::ExtensionExecuteMsg::SwapNonVaultFunds {
                    force_swap_route,
                    swap_routes,
                } => execute_swap_non_vault_funds(deps, env, info, force_swap_route, swap_routes),
                crate::msg::ExtensionExecuteMsg::CollectRewards {} => {
                    execute_collect_rewards(deps, env)
                }
                crate::msg::ExtensionExecuteMsg::MigrationStep { amount_of_users } => {
                    execute_migration_step(deps, env, amount_of_users)
                }
            }
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
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
            Ok(to_json_binary(&query_total_assets(deps, &env)?)?)
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
                crate::msg::UserBalanceQueryMsg::UserAssetsBalance { user } => {
                    Ok(to_json_binary(&query_user_assets(deps, env, user)?)?)
                }
            },
            crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(msg) => match msg {
                crate::msg::ClQueryMsg::Pool {} => Ok(to_json_binary(&query_pool(deps)?)?),
                crate::msg::ClQueryMsg::Positions {} => {
                    Ok(to_json_binary(&query_positions(deps)?)?)
                }
                crate::msg::ClQueryMsg::RangeAdmin {} => {
                    let range_admin = get_range_admin(deps)?;
                    Ok(to_json_binary(&RangeAdminResponse {
                        address: range_admin.to_string(),
                    })?)
                }
                crate::msg::ClQueryMsg::VerifyTickCache => {
                    Ok(to_json_binary(&query_verify_tick_cache(deps)?)?)
                }
                crate::msg::ClQueryMsg::MainPosition => {
                    Ok(to_json_binary(&query_main_position(deps)?)?)
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
        Replies::CollectIncentives => handle_collect_incentives_reply(deps, env, msg.result),
        Replies::CollectSpreadRewards => handle_collect_spread_rewards_reply(deps, env, msg.result),
        Replies::WithdrawPosition => handle_withdraw_position_reply(deps, env, msg.result),
        Replies::RangeInitialCreatePosition => {
            handle_initial_create_position_reply(deps, env, msg.result)
        }
        Replies::RangeIterationCreatePosition => {
            handle_iteration_create_position_reply(deps, env, msg.result)
        }
        Replies::RangeNewCreatePosition => handle_range_new_create_position(deps, env, msg.result),
        Replies::RangeAddToPosition => handle_range_add_to_position_reply(deps, env, msg.result),
        Replies::Swap => handle_swap_reply(deps, env, msg.result),
        Replies::Merge => handle_merge_reply(deps, env, msg.result),
        Replies::CreateDenom => handle_create_denom_reply(deps, msg.result),
        Replies::WithdrawUserMain => handle_withdraw_user_reply(deps, msg.result),
        Replies::WithdrawUserProRato => handle_withdraw_user_reply(deps, msg.result),
        Replies::WithdrawMerge => handle_merge_withdraw_position_reply(deps, env, msg.result),
        Replies::CreatePositionMerge => handle_merge_create_position_reply(deps, env, msg.result),
        Replies::Autocompound => handle_autocompound_reply(deps, env, msg.result),
        Replies::AnyDepositSwap => handle_any_deposit_swap_reply(deps, env, msg.result),
        Replies::Unknown => unimplemented!(),
    }
}

/// For migrating from single position to multirange, we need to take do the following:
/// - Take the current position of the vault
///   - Save that position as the main positon
///   - Save that position as a position in the POSITIONS map
/// - remove the position from the old key
/// For review verification, the following items are changed:
/// - POSITION (removed)
/// - POSITIONS (newly added)
/// - MAIN_POSITION_ID (newly added)
/// - CURRENT_POSITION_ID (newly added)
/// - CURRENT_CLAIM_AFTER (newly added)
/// - MERGE_MAIN_POSITION (newly added)
/// and claim_after_secs is added to
/// - CurrentMergePosition
/// - ModifyRangeState
/// 
/// Of these changed items CURRENT_POSITION_ID, CURRENT_CLAIM_AFTER, MERGE_MAIN_POSITION, CurrentMergePosition and ModifyRangeState
/// are set before they are are read, so do not need to be set in the migrations (reviewers should verify this).
/// This leaves us with the correct setting of POSITIONS, MAIN_POSITION_ID and the removal of POSITION
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    cw2::ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // position was left unaltered so we don't need to change this
    const POSITION: Item<Position> = Item::new("position");

    let position = POSITION.load(deps.storage)?;

    MAIN_POSITION_ID.save(deps.storage, &position.position_id)?;
    POSITIONS.save(deps.storage, position.position_id, &position)?;

    POSITION.remove(deps.storage);

    Ok(Response::new()
        .add_attribute("migrate", "succesful")
        .add_attribute("main_position", position.position_id.to_string())
        .add_attribute("contract_name", CONTRACT_NAME)
        .add_attribute("contract_version", CONTRACT_VERSION)
    )
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        coin,
        testing::{mock_dependencies, mock_env},
        Addr, Coin, CosmosMsg, Decimal, SubMsg, Uint128,
    };
    use cw2::assert_contract_version;
    use prost::Message;
    use std::str::FromStr;

    #[allow(deprecated)]
    use crate::{
        rewards::CoinList, state::USER_REWARDS,
        test_tube::initialize::initialize::MAX_SLIPPAGE_HIGH,
    };
    use crate::{
        state::OldVaultConfig,
        test_tube::initialize::initialize::{DENOM_BASE, DENOM_QUOTE, DENOM_REWARD},
    };
    use osmosis_std::{cosmwasm_to_proto_coins, types::cosmos::bank::v1beta1::MsgMultiSend};

    use super::*;

    #[test]
    fn migrate_positions_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        const POSITION: Item<Position> = Item::new("position");
        let position = Position { position_id: 2, join_time: 3, claim_after: None };
        POSITION.save(deps.as_mut().storage, &position).unwrap();
        
        cw2::set_contract_version(deps.as_mut().storage, CONTRACT_NAME, "0.0.0").unwrap();
        migrate(deps.as_mut(), env, MigrateMsg {  }).unwrap();

        assert!(!POSITION.exists(deps.as_ref().storage));
        assert_eq!(MAIN_POSITION_ID.load(deps.as_ref().storage).unwrap(), position.position_id);
        assert_eq!(POSITIONS.load(deps.as_ref().storage, position.position_id).unwrap(), position)
    }

    #[test]
    fn migrate_cw2_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        const POSITION: Item<Position> = Item::new("position");
        POSITION.save(deps.as_mut().storage, &Position { position_id: 2, join_time: 3, claim_after: None }).unwrap();

        cw2::set_contract_version(deps.as_mut().storage, CONTRACT_NAME, "0.0.0").unwrap();
        migrate(deps.as_mut(), env, MigrateMsg {  }).unwrap();

        assert_contract_version(deps.as_mut().storage, CONTRACT_NAME, CONTRACT_VERSION).unwrap();
    }
}
