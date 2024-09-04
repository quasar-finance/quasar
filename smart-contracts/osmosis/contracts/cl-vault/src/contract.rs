use crate::error::ContractError;
use crate::helpers::coinlist::CoinList;
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
    query_assets_from_shares, query_dex_router, query_info, query_metadata, query_pool,
    query_position, query_total_assets, query_total_vault_token_supply, query_user_assets,
    query_user_balance, query_verify_tick_cache, RangeAdminResponse,
};
use crate::reply::Replies;
use crate::state::{VaultConfig, VAULT_CONFIG};
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
use cosmwasm_schema::cw_serde;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Addr, Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Reply, Response,
};
use cw2::set_contract_version;
use cw_storage_plus::{Item, Map};
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
                    execute_admin(deps, info, admin_msg)
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
pub fn migrate(deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    let previous_version =
        cw2::ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let dex_router_item: Item<Addr> = Item::new("dex_router");
    dex_router_item.remove(deps.storage);
    // VaultConfig
    #[cw_serde]
    struct OldVaultConfig {
        pub performance_fee: Decimal,
        pub treasury: Addr,
        pub swap_max_slippage: Decimal,
        pub dex_router: Addr,
    }
    const OLD_VAULT_CONFIG: Item<OldVaultConfig> = Item::new("vault_config_v2");
    let old_vault_config: OldVaultConfig = OLD_VAULT_CONFIG.load(deps.storage)?;
    OLD_VAULT_CONFIG.remove(deps.storage);
    VAULT_CONFIG.save(
        deps.storage,
        &VaultConfig {
            performance_fee: old_vault_config.performance_fee,
            treasury: old_vault_config.treasury,
            swap_max_slippage: old_vault_config.swap_max_slippage,
            dex_router: old_vault_config.dex_router,
            swap_admin: msg.swap_admin,
            twap_window_seconds: msg.twap_window_seconds,
        },
    )?;

    // MigrationStatus
    #[cw_serde]
    pub enum MigrationStatus {
        Open,
        Closed,
    }
    pub const MIGRATION_STATUS: Item<MigrationStatus> = Item::new("migration_status");
    let migration_status = MIGRATION_STATUS.load(deps.storage)?;
    // we want the v1.0.8-skn migration_step to be occurred completely here.
    if migration_status == MigrationStatus::Open {
        return Err(ContractError::ParseError {
            msg: "Migration status should be closed.".to_string(),
        });
    }
    MIGRATION_STATUS.remove(deps.storage);

    // UserRewards
    pub const USER_REWARDS: Map<Addr, CoinList> = Map::new("user_rewards");
    USER_REWARDS.clear(deps.storage);

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
        assert!(set_contract_version(deps.as_mut().storage, CONTRACT_NAME, "0.3.0").is_ok());

        // VaultConfig mocking
        #[cw_serde]
        struct OldVaultConfig {
            pub performance_fee: Decimal,
            pub treasury: Addr,
            pub swap_max_slippage: Decimal,
            pub dex_router: Addr,
        }
        const OLD_VAULT_CONFIG: Item<OldVaultConfig> = Item::new("vault_config_v2");
        OLD_VAULT_CONFIG
            .save(
                deps.as_mut().storage,
                &OldVaultConfig {
                    performance_fee: Decimal::percent(1),
                    treasury: Addr::unchecked("treasury"),
                    swap_max_slippage: Decimal::percent(1),
                    dex_router: Addr::unchecked("dex_router"),
                },
            )
            .unwrap();

        // MigrationStatus mocking
        #[cw_serde]
        pub enum MigrationStatus {
            Open,
            Closed,
        }
        pub const MIGRATION_STATUS: Item<MigrationStatus> = Item::new("migration_status");
        MIGRATION_STATUS
            .save(deps.as_mut().storage, &MigrationStatus::Closed)
            .unwrap();

        // UserRewards mocking
        pub const USER_REWARDS: Map<Addr, CoinList> = Map::new("user_rewards");
        USER_REWARDS
            .save(
                deps.as_mut().storage,
                Addr::unchecked("user"),
                &CoinList::new(),
            )
            .unwrap();

        // Migrate and assert new states
        migrate(
            deps.as_mut(),
            env,
            MigrateMsg {
                swap_admin: Addr::unchecked("swap_admin"),
                twap_window_seconds: 24u64,
            },
        )
        .unwrap();

        let vault_config: VaultConfig = VAULT_CONFIG.load(&deps.storage).unwrap();
        assert_eq!(vault_config.performance_fee, Decimal::percent(1));
        assert_eq!(vault_config.treasury, Addr::unchecked("treasury"));
        assert_eq!(vault_config.swap_max_slippage, Decimal::percent(1));
        assert_eq!(vault_config.dex_router, Addr::unchecked("dex_router"));
        assert_eq!(vault_config.swap_admin, Addr::unchecked("swap_admin"));
        assert!(matches!(OLD_VAULT_CONFIG.may_load(&deps.storage), Ok(None)));

        assert!(matches!(MIGRATION_STATUS.may_load(&deps.storage), Ok(None)));

        assert!(matches!(
            USER_REWARDS.may_load(&deps.storage, Addr::unchecked("user")),
            Ok(None)
        ));
    }
}
