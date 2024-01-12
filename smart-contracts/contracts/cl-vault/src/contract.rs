use crate::error::{ContractError, ContractResult};
use crate::instantiate::{
    handle_create_denom_reply, handle_instantiate, handle_instantiate_create_position_reply,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, ModifyRangeMsg, QueryMsg};
use crate::query::{
    query_assets_from_shares, query_info, query_metadata, query_pool, query_position,
    query_total_assets, query_total_vault_token_supply, query_user_assets, query_user_balance,
    query_user_rewards, query_verify_tick_cache, RangeAdminResponse,
};
use crate::reply::Replies;
use crate::rewards::{
    execute_distribute_rewards, handle_collect_incentives_reply,
    handle_collect_spread_rewards_reply,
};
use std::str::FromStr;

use crate::helpers::get_unused_balances;
use crate::state::{
    MigrationData, PoolConfig, Position, CURRENT_BALANCE, MIGRATION_DATA, POOL_CONFIG, POSITION,
};
use crate::vault::admin::{execute_admin, execute_build_tick_exp_cache};
use crate::vault::claim::execute_claim_user_rewards;
use crate::vault::concentrated_liquidity::{create_position, get_position, withdraw_from_position};
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
    to_binary, Binary, Coin, Decimal256, Deps, DepsMut, Env, MessageInfo, Reply, Response, SubMsg,
    SubMsgResult, Uint128,
};
use cw2::set_contract_version;
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    MsgCreatePositionResponse, MsgWithdrawPositionResponse, Pool,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::PoolmanagerQuerier;

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
                }) => execute_update_range(
                    deps,
                    env,
                    info,
                    lower_price,
                    upper_price,
                    max_slippage,
                    ratio_of_swappable_funds_to_use,
                    twap_window_seconds,
                ),
                crate::msg::ExtensionExecuteMsg::DistributeRewards {} => {
                    execute_distribute_rewards(deps, env)
                }
                crate::msg::ExtensionExecuteMsg::ClaimRewards {} => {
                    execute_claim_user_rewards(deps, info.sender.as_str())
                }
                crate::msg::ExtensionExecuteMsg::BuildTickCache {} => {
                    execute_build_tick_exp_cache(deps, info)
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
            Ok(to_binary(&query_info(deps)?)?)
        }
        cw_vault_multi_standard::VaultStandardQueryMsg::PreviewDeposit { assets: _ } => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::DepositRatio => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::PreviewRedeem { amount: shares } => {
            Ok(to_binary(&query_assets_from_shares(deps, env, shares)?)?)
        }
        cw_vault_multi_standard::VaultStandardQueryMsg::TotalAssets {} => {
            Ok(to_binary(&query_total_assets(deps, env)?)?)
        }
        cw_vault_multi_standard::VaultStandardQueryMsg::TotalVaultTokenSupply {} => {
            Ok(to_binary(&query_total_vault_token_supply(deps)?)?)
        }
        cw_vault_multi_standard::VaultStandardQueryMsg::ConvertToShares { amount: _ } => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::ConvertToAssets { amount: shares } => {
            Ok(to_binary(&query_assets_from_shares(deps, env, shares)?)?)
        }
        cw_vault_multi_standard::VaultStandardQueryMsg::VaultExtension(msg) => match msg {
            crate::msg::ExtensionQueryMsg::Metadata {} => Ok(to_binary(&query_metadata(deps)?)?),
            crate::msg::ExtensionQueryMsg::Balances(msg) => match msg {
                crate::msg::UserBalanceQueryMsg::UserSharesBalance { user } => {
                    Ok(to_binary(&query_user_balance(deps, user)?)?)
                }
                crate::msg::UserBalanceQueryMsg::UserRewards { user } => {
                    Ok(to_binary(&query_user_rewards(deps, user)?)?)
                }
                crate::msg::UserBalanceQueryMsg::UserAssetsBalance { user } => {
                    Ok(to_binary(&query_user_assets(deps, env, user)?)?)
                }
            },
            crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(msg) => match msg {
                crate::msg::ClQueryMsg::Pool {} => Ok(to_binary(&query_pool(deps)?)?),
                crate::msg::ClQueryMsg::Position {} => Ok(to_binary(&query_position(deps)?)?),
                crate::msg::ClQueryMsg::RangeAdmin {} => {
                    let range_admin = get_range_admin(deps)?;
                    Ok(to_binary(&RangeAdminResponse {
                        address: range_admin.to_string(),
                    })?)
                }
                crate::msg::ClQueryMsg::VerifyTickCache => {
                    Ok(to_binary(&query_verify_tick_cache(deps)?)?)
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
        Replies::WithdrawToMigrate => handle_migration_withdrawal_reply(deps, env, msg.result),
        Replies::CreateMigratedPosition => {
            handle_create_migrated_position_reply(deps, env, msg.result)
        }
        Replies::Unknown => unimplemented!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    // Step 1: Withdraw current position from the existing pool
    let position_breakdown = get_position(deps.storage, &deps.querier)?;
    let position = position_breakdown.position.unwrap();

    // Prepare migration data
    let migration_data = MigrationData {
        new_pool_id: msg.pool_id,
        lower_tick: position.lower_tick,
        upper_tick: position.upper_tick,
    };
    MIGRATION_DATA.save(deps.storage, &migration_data)?;

    // Use the existing function to create the withdraw message
    let liquidity_amount = Decimal256::from_str(position.liquidity.as_str())?;
    let withdraw_msg = withdraw_from_position(deps.storage, &env, liquidity_amount)?;

    // Create the submessage for withdrawal
    Ok(Response::default()
        .add_submessage(SubMsg::reply_on_success(
            withdraw_msg,
            Replies::WithdrawToMigrate.into(),
        ))
        .add_attribute("migration", "initiated")
        .add_attribute("action", "modify_range")
        .add_attribute("method", "withdraw_position")
        .add_attribute("position_id", position.position_id.to_string())
        .add_attribute("liquidity_amount", position.liquidity))
}

fn handle_migration_withdrawal_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> ContractResult<Response> {
    //remove old positions
    POSITION.remove(deps.storage);

    let migration_data = MIGRATION_DATA.load(deps.storage)?;
    // Step 2: Verify pool assets if they match
    let new_pool: Pool = PoolmanagerQuerier::new(&deps.querier)
        .pool(migration_data.new_pool_id)?
        .pool
        .ok_or(ContractError::PoolNotFound {
            pool_id: migration_data.new_pool_id,
        })?
        .try_into()
        .unwrap();

    let pool_config = POOL_CONFIG.load(deps.storage)?;

    if (new_pool.token0 != pool_config.token0) || (new_pool.token1 != pool_config.token1) {
        return Err(ContractError::PoolTokenMismatch {});
    }

    //calculate total assets after withdrawal
    let msg: MsgWithdrawPositionResponse = data.try_into()?;

    let mut amount0: Uint128 = msg.amount0.parse()?;
    let mut amount1: Uint128 = msg.amount1.parse()?;

    let unused_balances = get_unused_balances(deps.storage, &deps.querier, &env)?;

    let unused_balance0 = unused_balances
        .find_coin(pool_config.token0.clone())
        .amount
        .checked_sub(amount0)?;
    let unused_balance1 = unused_balances
        .find_coin(pool_config.token1.clone())
        .amount
        .checked_sub(amount1)?;

    amount0 = amount0.checked_add(unused_balance0)?;
    amount1 = amount1.checked_add(unused_balance1)?;

    CURRENT_BALANCE.save(deps.storage, &(amount0, amount1))?;

    let mut tokens_provided = vec![];
    if !amount0.is_zero() {
        tokens_provided.push(Coin {
            denom: pool_config.token0,
            amount: amount0,
        })
    }
    if !amount1.is_zero() {
        tokens_provided.push(Coin {
            denom: pool_config.token1,
            amount: amount1,
        })
    }

    //save new pool info
    POOL_CONFIG.save(
        deps.storage,
        &PoolConfig {
            pool_id: new_pool.id,
            token0: new_pool.token0,
            token1: new_pool.token1,
        },
    )?;

    // create position in new pool
    let create_positon_msg = create_position(
        deps,
        &env,
        migration_data.lower_tick,
        migration_data.upper_tick,
        tokens_provided,
        Uint128::zero(),
        Uint128::zero(),
    )?;

    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(
            create_positon_msg,
            Replies::CreateMigratedPosition.into(),
        ))
        .add_attribute("migration", "in_progress")
        .add_attribute("position", "created"))
}

pub fn handle_create_migrated_position_reply(
    deps: DepsMut,
    _env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let response: MsgCreatePositionResponse = data.try_into()?;
    MIGRATION_DATA.remove(deps.storage);

    POSITION.save(
        deps.storage,
        &Position {
            position_id: response.position_id,
        },
    )?;

    Ok(Response::new().add_attribute("migration", "completed"))
}
