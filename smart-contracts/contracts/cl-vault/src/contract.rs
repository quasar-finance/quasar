#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, BankMsg, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response,
};
use cw2::set_contract_version;
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::ConcentratedliquidityQuerier;

use crate::error::ContractError;
use crate::helpers::generic::sort_tokens;
use crate::helpers::getters::get_range_admin;
use crate::helpers::prepend::prepend_claim_msg;
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
#[allow(deprecated)]
use crate::state::CURRENT_BALANCE;
#[allow(deprecated)]
use crate::state::{
    MigrationStatus, VaultConfig, MIGRATION_STATUS, OLD_VAULT_CONFIG, STRATEGIST_REWARDS,
    VAULT_CONFIG,
};
#[allow(deprecated)]
use crate::state::{Position, OLD_POSITION, POSITION};
use crate::vault::admin::execute_admin;
use crate::vault::any_deposit::{execute_any_deposit, handle_any_deposit_swap_reply};
use crate::vault::autocompound::{
    execute_autocompound, execute_migration_step, handle_autocompound_reply,
};
use crate::vault::distribution::{
    execute_collect_rewards, handle_collect_incentives_reply, handle_collect_spread_rewards_reply,
};
use crate::vault::exact_deposit::execute_exact_deposit;
use crate::vault::merge::{
    execute_merge_position, handle_merge_create_position_reply,
    handle_merge_withdraw_position_reply,
};
use crate::vault::range::{
    execute_update_range, handle_initial_create_position_reply,
    handle_iteration_create_position_reply, handle_merge_reply, handle_swap_reply,
    handle_withdraw_position_reply,
};
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
                crate::msg::ExtensionExecuteMsg::ModifyRange(ModifyRangeMsg {
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
                crate::msg::ExtensionExecuteMsg::SwapNonVaultFunds { swap_operations } => {
                    execute_swap_non_vault_funds(deps, env, info, swap_operations)
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
        Replies::WithdrawPosition => handle_withdraw_position_reply(deps, env),
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
    #[allow(deprecated)]
    let old_vault_config = OLD_VAULT_CONFIG.load(deps.storage)?;
    let new_vault_config = VaultConfig {
        performance_fee: old_vault_config.performance_fee,
        treasury: old_vault_config.treasury,
        swap_max_slippage: old_vault_config.swap_max_slippage,
        dex_router: deps.api.addr_validate(msg.dex_router.as_str())?,
    };

    #[allow(deprecated)]
    OLD_VAULT_CONFIG.remove(deps.storage);
    VAULT_CONFIG.save(deps.storage, &new_vault_config)?;

    MIGRATION_STATUS.save(deps.storage, &MigrationStatus::Open)?;

    // Declare response object as mut
    let mut response = Response::new().add_attribute("migrate", "successful");

    // Conditionally add a bank send message if the strategist rewards state is not empty
    #[allow(deprecated)]
    let strategist_rewards = STRATEGIST_REWARDS.load(deps.storage)?;
    if !strategist_rewards.is_empty() {
        let bank_send_msg = BankMsg::Send {
            to_address: new_vault_config.treasury.to_string(),
            amount: sort_tokens(strategist_rewards.coins()),
        };
        response = response.add_message(bank_send_msg);
    }
    // Remove the state
    #[allow(deprecated)]
    STRATEGIST_REWARDS.remove(deps.storage);

    //POSITION state migration
    #[allow(deprecated)]
    let old_position = OLD_POSITION.load(deps.storage)?;

    let cl_querier = ConcentratedliquidityQuerier::new(&deps.querier);
    let pos_response = cl_querier.position_by_id(old_position.position_id)?;

    let new_position: Position = Position {
        position_id: old_position.position_id,
        join_time: pos_response
            .position
            .unwrap()
            .position
            .unwrap()
            .join_time
            .unwrap()
            .seconds
            .unsigned_abs(),
        claim_after: None,
    };

    POSITION.save(deps.storage, &new_position)?;
    #[allow(deprecated)]
    OLD_POSITION.remove(deps.storage);
    #[allow(deprecated)]
    CURRENT_BALANCE.remove(deps.storage);

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(deprecated)]
    use crate::{
        helpers::coinlist::CoinList,
        state::{OldPosition, OldVaultConfig, Position, OLD_POSITION, POSITION},
        test_tube::initialize::initialize::{DENOM_BASE, DENOM_QUOTE, DENOM_REWARD},
    };
    #[allow(deprecated)]
    use crate::{state::USER_REWARDS, test_tube::initialize::initialize::MAX_SLIPPAGE_HIGH};
    use cosmwasm_std::{
        coin,
        testing::{mock_dependencies, mock_env},
        Addr, Coin, CosmosMsg, Decimal, SubMsg, Uint128,
    };
    use osmosis_std::{cosmwasm_to_proto_coins, types::cosmos::bank::v1beta1::MsgMultiSend};
    use prost::Message;
    use std::str::FromStr;

    pub fn mock_migrate(
        deps: DepsMut,
        _env: Env,
        msg: MigrateMsg,
    ) -> Result<Response, ContractError> {
        #[allow(deprecated)]
        let old_vault_config = OLD_VAULT_CONFIG.load(deps.storage)?;
        let new_vault_config = VaultConfig {
            performance_fee: old_vault_config.performance_fee,
            treasury: old_vault_config.treasury,
            swap_max_slippage: old_vault_config.swap_max_slippage,
            dex_router: deps.api.addr_validate(msg.dex_router.as_str())?,
        };

        #[allow(deprecated)]
        OLD_VAULT_CONFIG.remove(deps.storage);
        VAULT_CONFIG.save(deps.storage, &new_vault_config)?;

        MIGRATION_STATUS.save(deps.storage, &MigrationStatus::Open)?;

        // Declare response object as mut
        let mut response = Response::new().add_attribute("migrate", "successful");

        // Conditionally add a bank send message if the strategist rewards state is not empty
        #[allow(deprecated)]
        let strategist_rewards = STRATEGIST_REWARDS.load(deps.storage)?;
        if !strategist_rewards.is_empty() {
            let bank_send_msg = BankMsg::Send {
                to_address: new_vault_config.treasury.to_string(),
                amount: sort_tokens(strategist_rewards.coins()),
            };
            response = response.add_message(bank_send_msg);
        }
        // Remove the state
        #[allow(deprecated)]
        STRATEGIST_REWARDS.remove(deps.storage);
        #[allow(deprecated)]
        let old_position = OLD_POSITION.load(deps.storage)?;

        let new_position: Position = Position {
            position_id: old_position.position_id,
            join_time: 0,
            claim_after: None,
        };

        POSITION.save(deps.storage, &new_position)?;

        #[allow(deprecated)]
        OLD_POSITION.remove(deps.storage);
        #[allow(deprecated)]
        CURRENT_BALANCE.remove(deps.storage);

        Ok(response)
    }

    #[test]
    fn test_migrate_position_state() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let new_dex_router = Addr::unchecked("dex_router"); // new field nested in existing VaultConfig state

        // Mock a previous state item
        #[allow(deprecated)]
        OLD_POSITION
            .save(deps.as_mut().storage, &OldPosition { position_id: 1 })
            .unwrap();
        #[allow(deprecated)]
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
        #[allow(deprecated)]
        STRATEGIST_REWARDS
            .save(&mut deps.storage, &CoinList::new())
            .unwrap();

        mock_migrate(
            deps.as_mut(),
            env,
            MigrateMsg {
                dex_router: new_dex_router,
            },
        )
        .unwrap();

        let position = POSITION.load(deps.as_mut().storage).unwrap();

        assert_eq!(position.position_id, 1);
        assert_eq!(position.join_time, 0);
        assert!(position.claim_after.is_none());

        #[allow(deprecated)]
        let old_position = OLD_POSITION.may_load(deps.as_mut().storage).unwrap();
        assert!(old_position.is_none());

        #[allow(deprecated)]
        let current_balance = CURRENT_BALANCE.may_load(deps.as_mut().storage).unwrap();
        assert!(current_balance.is_none());
    }

    #[test]
    fn test_migrate_no_rewards() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        // Declare new items for states
        let new_dex_router = Addr::unchecked("dex_router"); // new field nested in existing VaultConfig state

        // Mock a previous state item
        #[allow(deprecated)]
        OLD_POSITION
            .save(deps.as_mut().storage, &OldPosition { position_id: 1 })
            .unwrap();
        #[allow(deprecated)]
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
        #[allow(deprecated)]
        STRATEGIST_REWARDS
            .save(&mut deps.storage, &CoinList::new())
            .unwrap();

        mock_migrate(
            deps.as_mut(),
            env.clone(),
            MigrateMsg {
                dex_router: new_dex_router.clone(),
            },
        )
        .unwrap();

        // Assert OLD_VAULT_CONFIG have been correctly removed by unwrapping the error
        #[allow(deprecated)]
        OLD_VAULT_CONFIG.load(deps.as_mut().storage).unwrap_err();

        // Assert new VAULT_CONFIG.dex_router field have correct value
        let vault_config = VAULT_CONFIG.load(deps.as_mut().storage).unwrap();
        assert_eq!(vault_config.dex_router, new_dex_router);

        // Assert new MIGRATION_STATUS state have correct value
        let migration_status = MIGRATION_STATUS.load(deps.as_mut().storage).unwrap();
        assert_eq!(migration_status, MigrationStatus::Open);

        // Assert STRATEGIST_REWARDS state have been correctly removed by unwrapping the error
        #[allow(deprecated)]
        STRATEGIST_REWARDS.load(deps.as_mut().storage).unwrap_err();

        // Execute one migration step and assert the correct behavior
        execute_migration_step(deps.as_mut(), env, Uint128::one()).unwrap();

        // Assert new MIGRATION_STATUS state have correct value
        let migration_status = MIGRATION_STATUS.load(deps.as_mut().storage).unwrap();
        assert_eq!(migration_status, MigrationStatus::Closed);
    }

    #[test]
    fn test_migrate_with_rewards_execute_steps() {
        let env = mock_env();
        let mut deps = mock_dependencies();

        // Declare new items for states
        let new_dex_router = Addr::unchecked("dex_router"); // new field nested in existing VaultConfig state

        // Mock a previous state item
        #[allow(deprecated)]
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

        #[allow(deprecated)]
        OLD_POSITION
            .save(deps.as_mut().storage, &OldPosition { position_id: 1 })
            .unwrap();

        // Mock USER_REWARDS in order to have something to iterate over
        let rewards_coins = vec![
            coin(1000u128, DENOM_BASE),
            coin(1000u128, DENOM_QUOTE),
            coin(1000u128, DENOM_REWARD),
        ];
        for i in 0..10 {
            #[allow(deprecated)]
            USER_REWARDS
                .save(
                    deps.as_mut().storage,
                    Addr::unchecked(format!("user{}", i)),
                    &CoinList::from_coins(rewards_coins.clone()),
                )
                .unwrap();
        }
        // Mock STRATEGIST_REWARDS in order to have something to distribute
        #[allow(deprecated)]
        STRATEGIST_REWARDS
            .save(
                deps.as_mut().storage,
                &CoinList::from_coins(rewards_coins.clone()),
            )
            .unwrap();

        let migrate_resp = mock_migrate(
            deps.as_mut(),
            env.clone(),
            MigrateMsg {
                dex_router: new_dex_router.clone(),
            },
        )
        .unwrap();

        if let Some(SubMsg {
            msg: CosmosMsg::Bank(BankMsg::Send { to_address, amount }),
            ..
        }) = migrate_resp.messages.get(0)
        {
            assert_eq!(to_address, "treasury");
            assert_eq!(amount, &rewards_coins);
        } else {
            panic!("Expected BankMsg::Send message in the response");
        }

        // Assert USER_REWARDS state have been correctly removed by unwrapping the error
        #[allow(deprecated)]
        STRATEGIST_REWARDS.load(deps.as_mut().storage).unwrap_err();

        // Assert OLD_VAULT_CONFIG have been correctly removed by unwrapping the error
        #[allow(deprecated)]
        OLD_VAULT_CONFIG.load(deps.as_mut().storage).unwrap_err();

        // Assert new VAULT_CONFIG.dex_router field have correct value
        let vault_config = VAULT_CONFIG.load(deps.as_mut().storage).unwrap();
        assert_eq!(vault_config.dex_router, new_dex_router);

        // Assert new MIGRATION_STATUS state have correct value
        let migration_status = MIGRATION_STATUS.load(deps.as_mut().storage).unwrap();
        assert_eq!(migration_status, MigrationStatus::Open);

        // Execute 9 migration steps paginating by 2 users_amount.
        // leaving the last user to close the migration in the last step
        for i in 0..9 {
            let migration_step =
                execute_migration_step(deps.as_mut(), env.clone(), Uint128::one()).unwrap();

            assert_multi_send(
                &migration_step.messages[0].msg,
                &format!("user{}", i),
                &rewards_coins,
            );

            // Assert new MIGRATION_STATUS state have correct value
            let migration_status = MIGRATION_STATUS.load(deps.as_mut().storage).unwrap();
            assert_eq!(migration_status, MigrationStatus::Open);
        }

        // Execute the last migration step
        let migration_step =
            execute_migration_step(deps.as_mut(), env.clone(), Uint128::one()).unwrap();
        assert_multi_send(
            &migration_step.messages[0].msg,
            &"user9".to_string(),
            &rewards_coins,
        );

        // Assert new MIGRATION_STATUS state have correct value
        let migration_status = MIGRATION_STATUS.load(deps.as_mut().storage).unwrap();
        assert_eq!(migration_status, MigrationStatus::Closed);

        // Assert USER_REWARDS state have been correctly removed by unwrapping the error
        for i in 0..10 {
            #[allow(deprecated)]
            USER_REWARDS
                .load(deps.as_mut().storage, Addr::unchecked(format!("user{}", i)))
                .unwrap_err();
        }
    }

    fn assert_multi_send(msg: &CosmosMsg, expected_user: &String, user_rewards_coins: &Vec<Coin>) {
        if let CosmosMsg::Stargate { type_url, value } = msg {
            // Decode and validate the MsgMultiSend message
            // This has been decoded manually rather than encoding the expected message because its simpler to assert the values
            assert_eq!(type_url, "/cosmos.bank.v1beta1.MsgMultiSend");
            let msg: MsgMultiSend =
                MsgMultiSend::decode(value.as_slice()).expect("Failed to decode MsgMultiSend");
            for output in msg.outputs {
                assert_eq!(&output.address, expected_user);
                assert_eq!(
                    output.coins,
                    cosmwasm_to_proto_coins(user_rewards_coins.iter().cloned())
                );
            }
        } else {
            panic!("Expected Stargate message type, found another.");
        }
    }
}
