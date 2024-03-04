use crate::error::{ContractError, ContractResult};
use crate::instantiate::{
    handle_create_denom_reply, handle_instantiate, handle_instantiate_create_position_reply,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::query::{
    query_convert_to_assets, query_full_positions, query_info, query_metadata, query_pool,
    query_positions, query_total_assets, query_total_vault_token_supply, query_user_balance,
    query_user_rewards,
};
use crate::reply::Replies;
use crate::rewards::{
    execute_callback_distribute_rewards, execute_distribute_rewards,
    handle_collect_incentives_reply, handle_collect_spread_rewards_reply,
};
use crate::state::{Position, POSITIONS};
use crate::vault::admin::execute_admin;
use crate::vault::claim::execute_claim_user_rewards;
use crate::vault::deposit::{
    execute_exact_deposit, execute_mint_callback, handle_deposit_create_position_reply,
};
// use crate::vault::deposit::{execute_exact_deposit, handle_deposit_create_position_reply};
use crate::vault::merge::{
    execute_merge, handle_merge_create_position_reply, handle_merge_withdraw_reply,
};
use crate::vault::range::create_position::handle_range_new_create_position;
use crate::vault::range::modify_percentage::handle_range_add_to_position_reply;
use crate::vault::range::move_position::{
    handle_initial_create_position_reply, handle_iteration_create_position_reply,
    handle_merge_response, handle_swap_reply, handle_withdraw_position_reply,
};
use crate::vault::range::update_range::execute_update_range;
use crate::vault::withdraw::{execute_withdraw, handle_withdraw_user_reply};
use cosmwasm_schema::cw_serde;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, Uint128,
};
use cw2::set_contract_version;
use cw_storage_plus::Item;

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
                crate::msg::ExtensionExecuteMsg::ModifyRange(msg) => {
                    execute_update_range(deps, env, info, msg)
                }
                crate::msg::ExtensionExecuteMsg::DistributeRewards {} => {
                    execute_distribute_rewards(deps, env)
                }
                crate::msg::ExtensionExecuteMsg::ClaimRewards {} => {
                    execute_claim_user_rewards(deps, info.sender.as_str())
                }
                crate::msg::ExtensionExecuteMsg::CallbackExecuteMsg(msg) => {
                    // only our contract is allowed to call callbacks
                    if env.contract.address != info.sender {
                        return Err(ContractError::Unauthorized {});
                    }
                    match msg {
                        crate::msg::CallbackExecuteMsg::DistributeRewards() => {
                            execute_callback_distribute_rewards(deps, env)
                        }
                        crate::msg::CallbackExecuteMsg::Merge(msg) => execute_merge(deps, env, msg),
                        crate::msg::CallbackExecuteMsg::MintUserDeposit {} => {
                            execute_mint_callback(deps, env)
                        }
                    }
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
        cw_vault_multi_standard::VaultStandardQueryMsg::PreviewRedeem { amount: _ } => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::TotalAssets {} => {
            Ok(to_json_binary(&query_total_assets(deps, env)?)?)
        }
        cw_vault_multi_standard::VaultStandardQueryMsg::TotalVaultTokenSupply {} => {
            Ok(to_json_binary(&query_total_vault_token_supply(deps)?)?)
        }
        cw_vault_multi_standard::VaultStandardQueryMsg::ConvertToShares { amount: _ } => todo!(),
        cw_vault_multi_standard::VaultStandardQueryMsg::ConvertToAssets { amount } => Ok(
            to_json_binary(&query_convert_to_assets(deps, env, amount)?)?,
        ),
        cw_vault_multi_standard::VaultStandardQueryMsg::VaultExtension(msg) => match msg {
            crate::msg::ExtensionQueryMsg::Metadata {} => {
                Ok(to_json_binary(&query_metadata(deps)?)?)
            }
            crate::msg::ExtensionQueryMsg::Balances(msg) => match msg {
                crate::msg::UserBalanceQueryMsg::UserSharesBalance { user } => {
                    Ok(to_json_binary(&query_user_balance(deps, user)?)?)
                }
                crate::msg::UserBalanceQueryMsg::UserRewards { user } => {
                    Ok(to_json_binary(&query_user_rewards(deps, user)?)?)
                }
            },
            crate::msg::ExtensionQueryMsg::ConcentratedLiquidity(msg) => match msg {
                crate::msg::ClQueryMsg::Pool {} => Ok(to_json_binary(&query_pool(deps)?)?),
                crate::msg::ClQueryMsg::Positions {} => {
                    Ok(to_json_binary(&query_positions(deps)?)?)
                }
                crate::msg::ClQueryMsg::RangeAdmin {} => todo!(),
                crate::msg::ClQueryMsg::FullPositions {} => {
                    Ok(to_json_binary(&query_full_positions(deps)?)?)
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
        Replies::CollectIncentives => handle_collect_incentives_reply(deps, msg.result),
        Replies::CollectSpreadRewards => handle_collect_spread_rewards_reply(deps, env, msg.result),
        Replies::WithdrawPosition => handle_withdraw_position_reply(deps, env, msg.result),
        Replies::RangeInitialCreatePosition => {
            handle_initial_create_position_reply(deps, env, msg.result)
        }
        Replies::RangeIterationCreatePosition => {
            handle_iteration_create_position_reply(deps, env, msg.result)
        }
        Replies::RangeAddToPosition => handle_range_add_to_position_reply(deps, env, msg.result),
        Replies::RangeNewCreatePosition => handle_range_new_create_position(deps, msg.result),
        Replies::Swap => handle_swap_reply(deps, env, msg.result),
        Replies::Merge => handle_merge_response(deps, msg.result),
        Replies::CreateDenom => handle_create_denom_reply(deps, msg.result),
        Replies::WithdrawUser => handle_withdraw_user_reply(deps, msg.result),
        Replies::WithdrawMerge => handle_merge_withdraw_reply(deps, env, msg.result),
        Replies::CreatePositionMerge => handle_merge_create_position_reply(deps, env, msg.result),
        Replies::Unknown => unimplemented!(),
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    // inline our previous state items
    #[cw_serde]
    pub struct OldPosition {
        pub position_id: u64,
    }

    pub const POSITION: Item<OldPosition> = Item::new("position");

    let position = POSITION.load(deps.storage)?;

    POSITIONS.save(
        deps.storage,
        position.position_id,
        &Position {
            position_id: position.position_id,
            ratio: Uint128::one(),
        },
    )?;

    Ok(Response::new().add_attribute("migrate", "successful"))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env};

    use super::*;

    #[test]
    fn migrate_works() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        // inline the old state items
        #[cw_serde]
        pub struct OldPosition {
            pub position_id: u64,
        }

        pub const POSITION: Item<OldPosition> = Item::new("position");
        let old = OldPosition { position_id: 1 };
        POSITION.save(deps.as_mut().storage, &old).unwrap();

        let migrate_msg = MigrateMsg {};

        migrate(deps.as_mut(), env, migrate_msg).unwrap();

        assert_eq!(
            POSITIONS
                .load(deps.as_ref().storage, old.position_id)
                .unwrap(),
            Position {
                position_id: old.position_id,
                ratio: Uint128::one()
            }
        );
    }
}
