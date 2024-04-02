use std::ops::Sub;

use apollo_cw_asset::AssetInfo;
use cosmwasm_std::{
    to_json_binary, Addr, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Order, QuerierWrapper,
    Response, Storage, SubMsg, Uint128, WasmMsg,
};
use cw_dex_router::{
    msg::{BestPathForPairResponse, ExecuteMsg as ApolloExecuteMsg, QueryMsg as ApolloQueryMsg},
    operations::SwapOperationsListUnchecked,
};
use osmosis_std::types::cosmos::bank::v1beta1::{Input, MsgMultiSend, Output};

use crate::state::{
    MigrationStatus, AUTO_COMPOUND_ADMIN, MIGRATION_STATUS, POOL_CONFIG, USER_REWARDS,
};
use crate::{
    msg::AutoCompoundAsset,
    state::{STRATEGIST_REWARDS, VAULT_CONFIG},
    ContractError,
};

use super::helpers::CoinList;

// Migration is a to-depreacate entrypoint useful to migrate from Distribute to Accumulate after Autocompound implementation
pub fn execute_migration_step(
    deps: DepsMut,
    env: Env,
    amount_of_users: Uint128,
) -> Result<Response, ContractError> {
    let mut migration_status = MIGRATION_STATUS.load(deps.storage)?;

    if matches!(migration_status, MigrationStatus::Closed) {
        return Err(ContractError::MigrationStatusClosed {});
    }

    let mut outputs = Vec::new();
    let mut addresses = Vec::new();
    let mut total_amount = CoinList::new();

    // Iterate user rewards in a paginated fashion
    for item in USER_REWARDS
        .range(deps.storage, None, None, Order::Ascending)
        .take(amount_of_users.u128() as usize)
    {
        let (address, rewards) = item?;
        deps.api.debug(format!("address {:?}", address).as_str());
        deps.api.debug(format!("rewards {:?}", rewards).as_str());

        addresses.push(address.clone());
        outputs.push(Output {
            address: address.to_string(),
            coins: rewards.osmo_coin_from_coin_list(),
        });
        total_amount.add(rewards)?;
    }
    deps.api
        .debug(format!("total_amount {:?}", total_amount).as_str());

    // Remove processed rewards in a separate iteration.
    for addr in addresses {
        USER_REWARDS.remove(deps.storage, addr);
    }

    // Check if this is the last execution.
    let is_last_execution = USER_REWARDS
        .range(deps.storage, None, None, Order::Ascending)
        .next()
        .is_none();
    deps.api
        .debug(format!("is_last_execution {:?}", is_last_execution).as_str());

    if is_last_execution {
        deps.api.debug("{:?}");
        migration_status = MigrationStatus::Closed;
        MIGRATION_STATUS.save(deps.storage, &migration_status)?;
    }

    let mut response = Response::new();
    // Only if there are rewards append the send_message
    if !total_amount.is_empty() {
        let send_message = MsgMultiSend {
            inputs: vec![Input {
                address: env.contract.address.to_string(),
                coins: total_amount.osmo_coin_from_coin_list(),
            }],
            outputs,
        };
        response = response.add_message(send_message);
    }
    response = response
        .add_attribute("migration_status", format!("{:?}", migration_status))
        .add_attribute("is_last_execution", is_last_execution.to_string());

    Ok(response)
}

pub fn execute_auto_compound_swap(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    force_swap_route: bool,
    swap_routes: Vec<AutoCompoundAsset>,
) -> Result<Response, ContractError> {
    // auto compound admin
    assert_auto_compound_admin(deps.storage, &info.sender)?;

    let vault_config = VAULT_CONFIG.may_load(deps.storage)?;
    let dex_router = vault_config.unwrap().dex_router;
    if swap_routes.is_empty() {
        return Err(ContractError::EmptyCompoundAssetList {});
    }

    let mut swap_msgs: Vec<SubMsg> = vec![];
    for current_swap_route in swap_routes {
        // sanity check on the amount of tokens
        let strategist_rewards = STRATEGIST_REWARDS.load(deps.storage)?;
        let balance_in_contract = deps.querier.query_balance(
            env.clone().contract.address,
            current_swap_route.clone().token_in_denom,
        )?;
        let balance_remaining_for_swap = balance_in_contract.amount.sub(
            strategist_rewards
                .find_coin(current_swap_route.clone().token_in_denom)
                .amount,
        );

        // todo ask if this check is needed
        if balance_remaining_for_swap == Uint128::zero() {
            return Err(ContractError::InsufficientFundsForSwap {
                balance: balance_in_contract.amount,
                needed: strategist_rewards
                    .find_coin(current_swap_route.clone().token_in_denom)
                    .amount,
            });
        }

        let pool_config = POOL_CONFIG.load(deps.storage)?;

        let part_1_amount = balance_remaining_for_swap
            .checked_div(Uint128::new(2))
            .unwrap();
        let part_2_amount = balance_remaining_for_swap
            .checked_add(Uint128::new(1))
            .unwrap()
            .checked_div(Uint128::new(2))
            .unwrap();

        swap_msgs.push(SubMsg::new(generate_swap_message(
            deps.querier,
            Some(dex_router.clone()),
            current_swap_route.clone().recommended_swap_route_token_0,
            current_swap_route.clone().token_in_denom,
            part_1_amount,
            pool_config.token0,
            force_swap_route,
        )?));
        swap_msgs.push(SubMsg::new(generate_swap_message(
            deps.querier,
            Some(dex_router.clone()),
            current_swap_route.clone().recommended_swap_route_token_1,
            current_swap_route.clone().token_in_denom,
            part_2_amount,
            pool_config.token1,
            force_swap_route,
        )?));
    }

    Ok(Response::new()
        .add_submessages(swap_msgs)
        .add_attribute("method", "execute")
        .add_attribute("action", "auto_compund_swap"))
}

fn generate_swap_message(
    querier: QuerierWrapper,
    dex_router: Option<Addr>,
    current_swap_route: Option<SwapOperationsListUnchecked>,
    token_in_denom: String,
    token_in_amount: Uint128,
    token_out_denom: String,
    force_swap_route: bool,
) -> Result<CosmosMsg, ContractError> {
    let swap_msg = match dex_router {
        Some(ref dex_router_address) => {
            let offer_asset = AssetInfo::Native(token_in_denom.clone());
            let ask_asset = AssetInfo::Native(token_out_denom);

            let recommended_out: Uint128 = match current_swap_route.clone() {
                Some(operations) => querier.query_wasm_smart(
                    dex_router_address.to_string(),
                    &ApolloQueryMsg::SimulateSwapOperations {
                        offer_amount: token_in_amount,
                        operations,
                    },
                )?,
                None => 0u128.into(),
            };
            let best_path: Option<BestPathForPairResponse> = querier.query_wasm_smart(
                dex_router_address.to_string(),
                &ApolloQueryMsg::BestPathForPair {
                    offer_asset: offer_asset.into(),
                    ask_asset: ask_asset.into(),
                    exclude_paths: None,
                    offer_amount: token_in_amount,
                },
            )?;
            let best_outcome = best_path
                .as_ref()
                .map_or(Uint128::zero(), |path| path.return_amount);

            // Determine the route to use
            let route = if force_swap_route {
                current_swap_route
                    .clone()
                    .ok_or(ContractError::TryForceRouteWithoutRecommendedSwapRoute {})?
            } else if best_outcome >= recommended_out {
                best_path.expect("Expected a best path").operations.into()
            } else {
                current_swap_route
                    .clone()
                    .expect("Expected a recommended route")
            };

            // Execute swap operations once with the determined route
            get_execute_swap_operations_msg(
                dex_router_address.clone(),
                route,
                Uint128::zero(),
                &token_in_denom.clone(),
                token_in_amount,
            )
        }
        None => {
            return Err(ContractError::InvalidDexRouterAddress {});
        }
    };

    swap_msg
}

fn get_execute_swap_operations_msg(
    dex_router_address: Addr,
    operations: SwapOperationsListUnchecked,
    token_out_min_amount: Uint128,
    token_in_denom: &String,
    token_in_amount: Uint128,
) -> Result<CosmosMsg, ContractError> {
    let swap_msg: CosmosMsg = WasmMsg::Execute {
        contract_addr: dex_router_address.to_string(),
        msg: to_json_binary(&ApolloExecuteMsg::ExecuteSwapOperations {
            operations,
            minimum_receive: Some(token_out_min_amount),
            to: None,
            offer_amount: None,
        })?,
        funds: vec![Coin {
            denom: token_in_denom.to_string(),
            amount: token_in_amount,
        }],
    }
    .into();

    Ok(swap_msg)
}

fn assert_auto_compound_admin(
    storage: &mut dyn Storage,
    sender: &Addr,
) -> Result<(), ContractError> {
    let admin = AUTO_COMPOUND_ADMIN.load(storage)?;
    if admin != sender {
        return Err(ContractError::Unauthorized {});
    }
    Ok(())
}
