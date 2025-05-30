use crate::error::ContractError;
use crate::helpers::getters::get_range_admin;
use crate::helpers::prepend::prepend_claim_msg;
use crate::instantiate::{
    handle_create_denom_reply, handle_instantiate, handle_instantiate_create_position_reply,
};
use crate::msg::{
    ClQueryMsg, ExecuteMsg, ExtensionExecuteMsg, ExtensionQueryMsg, InstantiateMsg, MigrateMsg,
    ModifyRangeMsg, QueryMsg,
};
use crate::query::{
    query_active_users, query_assets_from_shares, query_dex_router, query_info, query_metadata,
    query_pool, query_position, query_total_assets, query_total_vault_token_supply,
    query_user_assets, query_user_balance, query_verify_tick_cache, RangeAdminResponse,
};
use crate::reply::Replies;
use crate::vault::{
    admin::execute_admin,
    autocompound::{execute_autocompound, handle_autocompound_reply, handle_merge_reply},
    deposit::{execute_any_deposit, execute_exact_deposit, handle_any_deposit_swap_reply},
    distribution::{
        execute_collect_rewards, handle_collect_incentives_reply,
        handle_collect_spread_rewards_reply,
    },
    merge::{
        execute_merge_position, handle_merge_create_position_reply,
        handle_merge_withdraw_position_reply,
    },
    range::{
        execute_update_range, handle_create_position, handle_swap_reply,
        handle_withdraw_position_reply,
    },
    swap::execute_swap_non_vault_funds,
    withdraw::{execute_withdraw, handle_withdraw_user_reply},
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response};
use cw2::set_contract_version;
use quasar_types::cw_vault_multi_standard::{VaultStandardExecuteMsg, VaultStandardQueryMsg};
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
        VaultStandardExecuteMsg::AnyDeposit {
            recipient,
            max_slippage,
            .. // asset and amount fields are not used in this implementation, they are for CW20 tokens
        } => execute_any_deposit(deps, env, info, recipient, max_slippage),
        VaultStandardExecuteMsg::ExactDeposit { recipient } => {
            execute_exact_deposit(deps, env, info, recipient)
        }
        VaultStandardExecuteMsg::Redeem { recipient, amount } => {
            prepend_claim_msg(
                &env,
                execute_withdraw(deps, &env, info, recipient, amount.into())?,
            )
        }
        VaultStandardExecuteMsg::VaultExtension(vault_msg) => {
            match vault_msg {
                ExtensionExecuteMsg::Admin(admin_msg) => {
                    execute_admin(deps, env, info, admin_msg)
                }
                ExtensionExecuteMsg::Merge(msg) => {
                    execute_merge_position(deps, env, info, msg)
                }
                ExtensionExecuteMsg::Autocompound {} => {
                    prepend_claim_msg(&env, execute_autocompound(deps, &env, info)?)
                }
                ExtensionExecuteMsg::ModifyRange(ModifyRangeMsg {
                    lower_price,
                    upper_price,
                    max_slippage,
                    ratio_of_swappable_funds_to_use,
                    twap_window_seconds,
                    forced_swap_route,
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
                        forced_swap_route,
                        claim_after,
                    )?,
                ),
                ExtensionExecuteMsg::SwapNonVaultFunds {
                    swap_operations,
                    twap_window_seconds,
                } => execute_swap_non_vault_funds(deps, env, info, swap_operations, twap_window_seconds),
                ExtensionExecuteMsg::CollectRewards {} => {
                    execute_collect_rewards(deps, env)
                }
            }
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        VaultStandardQueryMsg::VaultStandardInfo {} => {
            todo!()
        }
        VaultStandardQueryMsg::Info {} => Ok(to_json_binary(&query_info(deps)?)?),
        VaultStandardQueryMsg::PreviewDeposit { assets: _ } => todo!(),
        VaultStandardQueryMsg::DepositRatio => todo!(),
        VaultStandardQueryMsg::PreviewRedeem { amount: shares } => Ok(to_json_binary(
            &query_assets_from_shares(deps, env, shares)?,
        )?),
        VaultStandardQueryMsg::TotalAssets {} => {
            Ok(to_json_binary(&query_total_assets(deps, env)?)?)
        }
        VaultStandardQueryMsg::TotalVaultTokenSupply {} => {
            Ok(to_json_binary(&query_total_vault_token_supply(deps)?)?)
        }
        VaultStandardQueryMsg::ConvertToShares { amount: _ } => todo!(),
        VaultStandardQueryMsg::ConvertToAssets { amount: shares } => Ok(to_json_binary(
            &query_assets_from_shares(deps, env, shares)?,
        )?),
        VaultStandardQueryMsg::VaultExtension(msg) => match msg {
            ExtensionQueryMsg::Metadata {} => Ok(to_json_binary(&query_metadata(deps)?)?),
            ExtensionQueryMsg::DexRouter {} => Ok(to_json_binary(&query_dex_router(deps)?)?),
            ExtensionQueryMsg::Balances(msg) => match msg {
                crate::msg::UserBalanceQueryMsg::UserSharesBalance { user } => {
                    Ok(to_json_binary(&query_user_balance(deps, user)?)?)
                }
                crate::msg::UserBalanceQueryMsg::UserAssetsBalance { user } => {
                    Ok(to_json_binary(&query_user_assets(deps, env, user)?)?)
                }
            },
            ExtensionQueryMsg::ConcentratedLiquidity(msg) => match msg {
                ClQueryMsg::Pool {} => Ok(to_json_binary(&query_pool(deps)?)?),
                ClQueryMsg::Position {} => Ok(to_json_binary(&query_position(deps)?)?),
                ClQueryMsg::RangeAdmin {} => {
                    let range_admin = get_range_admin(deps)?;
                    Ok(to_json_binary(&RangeAdminResponse {
                        address: range_admin.to_string(),
                    })?)
                }
                ClQueryMsg::VerifyTickCache => Ok(to_json_binary(&query_verify_tick_cache(deps)?)?),
            },
            ExtensionQueryMsg::Users {
                start_bound_exclusive,
                limit,
            } => Ok(to_json_binary(&query_active_users(
                deps,
                start_bound_exclusive.map(|s| deps.api.addr_validate(s.as_str()).unwrap()),
                limit,
            )?)?),
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
        Replies::WithdrawPosition => handle_withdraw_position_reply(deps, env),
        Replies::CreatePosition => handle_create_position(deps, env, msg.result),
        Replies::Swap => handle_swap_reply(deps, env),
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
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let previous_version =
        cw2::ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let response = Response::new()
        .add_attribute("migrate", "successful")
        .add_attribute("previous version", previous_version.to_string())
        .add_attribute("new version", CONTRACT_VERSION);
    Ok(response)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env};

    use super::*;

    #[test]
    fn test_migrate() {
        let env = mock_env();
        let mut deps = mock_dependencies();

        // Set an older version
        assert!(set_contract_version(deps.as_mut().storage, CONTRACT_NAME, "0.3.0").is_ok());

        // Perform migration
        let result = migrate(deps.as_mut(), env, MigrateMsg {});

        // Assert migration was successful
        assert!(result.is_ok());
        let response = result.unwrap();

        // Check response attributes
        assert_eq!(response.attributes.len(), 3);
        assert_eq!(response.attributes[0].key, "migrate");
        assert_eq!(response.attributes[0].value, "successful");
        assert_eq!(response.attributes[1].key, "previous version");
        assert_eq!(response.attributes[1].value, "0.3.0");
        assert_eq!(response.attributes[2].key, "new version");
        assert_eq!(response.attributes[2].value, CONTRACT_VERSION);

        // Verify contract version was updated
        let version = cw2::get_contract_version(&deps.storage).unwrap();
        assert_eq!(version.contract, CONTRACT_NAME);
        assert_eq!(version.version, CONTRACT_VERSION);
    }
}
