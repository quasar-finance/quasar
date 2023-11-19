use crate::error::{ContractError, ContractResult};
use crate::instantiate::{
    handle_create_denom_reply, handle_instantiate, handle_instantiate_create_position_reply,
};
use crate::math::tick::purge_tick_exp_cache;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, ModifyRangeMsg, QueryMsg};
use crate::query::{
    query_assets_from_shares, query_info, query_metadata, query_pool, query_position,
    query_total_assets, query_total_vault_token_supply, query_user_assets, query_user_balance,
    query_user_rewards, RangeAdminResponse,
};
use crate::reply::Replies;
use crate::rewards::{
    execute_distribute_rewards, handle_collect_incentives_reply,
    handle_collect_spread_rewards_reply,
};
use crate::state::{SHARES, VAULT_DENOM};
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
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, Uint128, Uint256,
};
use cw2::set_contract_version;
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmoCoin;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgBurn;

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
pub fn migrate(deps: DepsMut, env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let vault_denom = VAULT_DENOM.load(deps.storage)?;
    let mut response = Response::new();

    let vals: Result<Vec<(Addr, Uint128)>, ContractError> = SHARES
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|val| -> Result<(Addr, Uint128), ContractError> {
            let (user, shares) = val?;
            // Convert Uint128 to Uint256 for old shares
            let shares_256 = Uint256::from(shares.u128());

            // Perform the division
            let new_shares_256 = shares_256
                .checked_div(Uint256::from_u128(10u128).pow(18))
                .expect("Underflow");

            // Convert back to Uint128 for new shares, handling potential overflow
            let new_shares: Uint128 = Uint128::try_from(new_shares_256)
                .expect("Conversion from Uint256 to Uint128 failed due to overflow");

            let burn_amount_user =
                Uint128::try_from(shares_256.checked_sub(Uint256::from(new_shares.u128()))?)
                    .expect("Overflow/Underflow in burn amount calculation for user");

            if burn_amount_user.gt(&Uint128::zero()) {
                // Create a burn message for each user
                let individual_burn = MsgBurn {
                    amount: Some(OsmoCoin {
                        amount: burn_amount_user.to_string(),
                        denom: vault_denom.clone(),
                    }),
                    sender: env.contract.address.to_string(),
                    burn_from_address: env.contract.address.to_string(),
                };

                // Add the burn message to the response
                response = response.clone().add_message(individual_burn);
            }

            Ok((user, new_shares))
        })
        .collect();

    for (user, new_shares) in vals? {
        SHARES.save(deps.storage, user, &new_shares)?;
    }

    response = response.add_attribute("migrate", "successful");

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{Addr, Uint128};

    #[test]
    fn test_migration() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        VAULT_DENOM.save(
            deps.as_mut().storage,
            &String::from("tokenfactory/test/migration"),
        );

        let user_shares = [
            ("user1", 334336393064997666210680868153120255847u128),
            ("user2", 3861723043195065215677860659781464267u128),
            ("user3", 5207698598937150446323936506894565090u128),
            ("user4", 100926648309343424566468024997861860u128),
            ("user5", 89285540372043010706153009474481050u128),
            ("user6", 0u128),
            ("user7", 19497052606527749357594751011469352u128),
        ];

        for (user, shares) in user_shares.iter() {
            let addr = Addr::unchecked(*user);
            let initial_shares = Uint128::new(*shares);
            SHARES
                .save(deps.as_mut().storage, addr.clone(), &initial_shares)
                .unwrap();
        }

        let migrate_msg = MigrateMsg {};
        let migrate_response = migrate(deps.as_mut(), env.clone(), migrate_msg).unwrap();

        assert_eq!(migrate_response.attributes[0].key, "migrate");
        assert_eq!(migrate_response.attributes[0].value, "successful");

        for (user, shares) in user_shares.iter() {
            let addr = Addr::unchecked(*user);
            let updated_shares = SHARES.load(deps.as_ref().storage, addr.clone()).unwrap();
            let expected_shares = Uint128::new(*shares) / Uint128::new(10u128).pow(18);
            assert_eq!(updated_shares, expected_shares);
        }

        let messages = migrate_response.messages.clone();
        assert_eq!(messages.len(), 6);
    }
}
