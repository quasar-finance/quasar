use crate::error::ContractError;
use crate::helpers::sort_tokens;
use crate::instantiate::{
    handle_create_denom_reply, handle_instantiate, handle_instantiate_create_position_reply,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, ModifyRangeMsg, QueryMsg};
use crate::query::{
    query_assets_from_shares, query_dex_router, query_info, query_metadata, query_pool,
    query_position, query_total_assets, query_total_vault_token_supply, query_user_assets,
    query_user_balance, query_verify_tick_cache, RangeAdminResponse,
};
use crate::reply::Replies;
use crate::rewards::{
    execute_collect_rewards, handle_collect_incentives_reply, handle_collect_spread_rewards_reply,
    prepend_claim_msg,
};
use crate::vault::admin::{execute_admin, execute_build_tick_exp_cache};

use crate::state::{
    MigrationStatus, VaultConfig, AUTO_COMPOUND_ADMIN, MIGRATION_STATUS, OLD_VAULT_CONFIG,
    STRATEGIST_REWARDS, VAULT_CONFIG,
};
use crate::vault::any_deposit::{execute_any_deposit, handle_any_deposit_swap_reply};
use crate::vault::autocompound::{
    execute_autocompound, execute_migration_step, handle_autocompound_reply,
};
use crate::vault::exact_deposit::execute_exact_deposit;
use crate::vault::merge::{
    execute_merge_position, handle_merge_create_position_reply,
    handle_merge_withdraw_position_reply,
};
use crate::vault::range::{
    execute_update_range, get_range_admin, handle_initial_create_position_reply,
    handle_iteration_create_position_reply, handle_merge_reply, handle_swap_reply,
    handle_withdraw_position_reply,
};
use crate::vault::swap::execute_swap_non_vault_funds;
use crate::vault::withdraw::{execute_withdraw, handle_withdraw_user_reply};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response,
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
            recipient,
        } => execute_any_deposit(deps, env, info, recipient),
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
                crate::msg::ExtensionExecuteMsg::Merge(msg) => {
                    execute_merge_position(deps, env, info, msg)
                }
                crate::msg::ExtensionExecuteMsg::Autocompound {} => {
                    prepend_claim_msg(&env, execute_autocompound(deps, &env, info)?)
                }
                crate::msg::ExtensionExecuteMsg::ModifyRange(ModifyRangeMsg {
                    lower_price,
                    upper_price,
                    max_slippage,
                    ratio_of_swappable_funds_to_use,
                    twap_window_seconds,
                    recommended_swap_route,
                    force_swap_route,
                    claim_after,
                }) => prepend_claim_msg(
                    &env,
                    execute_update_range(
                        deps,
                        &env,
                        info,
                        lower_price,
                        upper_price,
                        max_slippage,
                        ratio_of_swappable_funds_to_use,
                        twap_window_seconds,
                        recommended_swap_route,
                        force_swap_route,
                        claim_after,
                    )?,
                ),
                crate::msg::ExtensionExecuteMsg::SwapNonVaultFunds {
                    force_swap_route,
                    swap_routes,
                } => execute_swap_non_vault_funds(deps, env, info, force_swap_route, swap_routes),
                crate::msg::ExtensionExecuteMsg::BuildTickCache {} => {
                    execute_build_tick_exp_cache(deps, info)
                }
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
        Replies::Merge => handle_merge_reply(deps, env, msg.result),
        Replies::CreateDenom => handle_create_denom_reply(deps, msg.result),
        Replies::WithdrawUser => handle_withdraw_user_reply(deps, msg.result),
        Replies::WithdrawMerge => handle_merge_withdraw_position_reply(deps, env, msg.result),
        Replies::CreatePositionMerge => handle_merge_create_position_reply(deps, env, msg.result),
        Replies::Autocompound => handle_autocompound_reply(deps, env, msg.result),
        Replies::AnyDepositSwap => handle_any_deposit_swap_reply(deps, env, msg.result),
        Replies::Unknown => unimplemented!(),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    let old_vault_config = OLD_VAULT_CONFIG.load(deps.storage)?;
    let new_vault_config = VaultConfig {
        performance_fee: old_vault_config.performance_fee,
        treasury: old_vault_config.treasury,
        swap_max_slippage: old_vault_config.swap_max_slippage,
        dex_router: deps.api.addr_validate(msg.dex_router.as_str())?,
    };

    OLD_VAULT_CONFIG.remove(deps.storage);
    VAULT_CONFIG.save(deps.storage, &new_vault_config)?;

    AUTO_COMPOUND_ADMIN.save(
        deps.storage,
        &deps.api.addr_validate(msg.auto_compound_admin.as_ref())?,
    )?;

    MIGRATION_STATUS.save(deps.storage, &MigrationStatus::Open)?;

    // Declare response object as mut
    let mut response = Response::new().add_attribute("migrate", "successful");

    // Conditionally add a bank send message if the strategist rewards state is not empty
    let strategist_rewards = STRATEGIST_REWARDS.load(deps.storage)?;
    if !strategist_rewards.is_empty() {
        let bank_send_msg = BankMsg::Send {
            to_address: new_vault_config.treasury.to_string(),
            amount: sort_tokens(strategist_rewards.coins()),
        };
        response = response.add_message(bank_send_msg);
    }
    // Remove the state
    STRATEGIST_REWARDS.remove(deps.storage);

    Ok(response)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env},
        Addr, Decimal,
    };
    use std::str::FromStr;

    use crate::state::OldVaultConfig;
    use crate::test_tube::initialize::initialize::MAX_SLIPPAGE_HIGH;

    use super::*;

    #[test]
    fn test_migrate() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        // Declare new items for states
        let new_dex_router = Addr::unchecked("dex_router"); // new field nested in existing VaultConfig state
        let new_auto_compound_admin = Addr::unchecked("auto_compound_admin"); // completely new state item

        // Mock a previous state item
        OLD_VAULT_CONFIG
            .save(
                deps.as_mut().storage,
                &OldVaultConfig {
                    performance_fee: Decimal::from_str("0.2").unwrap(),
                    treasury: Addr::unchecked("treasury"),
                    swap_max_slippage: Decimal::bps(MAX_SLIPPAGE_HIGH),
                },
            )
            .unwrap();

        let _ = migrate(
            deps.as_mut(),
            env,
            MigrateMsg {
                dex_router: new_dex_router.clone(),
                auto_compound_admin: new_auto_compound_admin.clone(),
            },
        );

        // Assert OLD_VAULT_CONFIG have been correctly removed by unwrapping the error
        OLD_VAULT_CONFIG.load(deps.as_mut().storage).unwrap_err();

        // Assert new VAULT_CONFIG.dex_router field have correct value
        let vault_config = VAULT_CONFIG.load(deps.as_mut().storage).unwrap();
        assert_eq!(vault_config.dex_router, new_dex_router);

        // Assert new AUTO_COMPOUND_ADMIN state have correct value
        let auto_compound_admin = AUTO_COMPOUND_ADMIN.load(deps.as_mut().storage).unwrap();
        assert_eq!(auto_compound_admin, new_auto_compound_admin);

        // Assert new MIGRATION_STATUS state have correct value
        let migration_status = MIGRATION_STATUS.load(deps.as_mut().storage).unwrap();
        assert_eq!(migration_status, MigrationStatus::Open);

        // TODO: Add bankSend check in this test
    }
}
