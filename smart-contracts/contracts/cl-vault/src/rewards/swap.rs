use apollo_cw_asset::AssetInfo;
use cosmwasm_std::{
    to_json_binary, Addr, Coin, CosmosMsg, DepsMut, Env, MessageInfo, QuerierWrapper, Response,
    SubMsg, Uint128, WasmMsg,
};
use cw_dex_router::{
    msg::{BestPathForPairResponse, ExecuteMsg as ApolloExecuteMsg, QueryMsg as ApolloQueryMsg},
    operations::SwapOperationsListUnchecked,
};

use crate::{helpers::assert_auto_compound_admin, state::POOL_CONFIG};
use crate::{msg::SwapAsset, state::VAULT_CONFIG, ContractError};

// TODO: I would like to rename this to a more generic thing like "execute_idle_funds_swap" or just "execute_swap"
pub fn execute_swap_idle_funds(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    force_swap_route: bool,
    swap_routes: Vec<SwapAsset>,
) -> Result<Response, ContractError> {
    // TODO: Should we assert that no BASE_DENOM / QUOTE_DENOM is trying to be swapped?
    //       Idt there is any use case for that and so we'd prevent tempering and/or fat fingering.

    // auto compound admin as the purpose of swaps are mainly around autocompound non-vault assets into assets that can be actually compounded.
    assert_auto_compound_admin(deps.storage, &info.sender)?;

    let vault_config = VAULT_CONFIG.may_load(deps.storage)?;
    let dex_router = vault_config.unwrap().dex_router;
    if swap_routes.is_empty() {
        return Err(ContractError::EmptyCompoundAssetList {});
    }

    // TODO: Why are we using SubMsgs if we are not replying?
    //       We should probably use Messages to fire and forget with atomic behavior?
    let mut swap_msgs: Vec<SubMsg> = vec![];
    for current_swap_route in swap_routes {
        let balance_in_contract = deps
            .querier
            .query_balance(
                env.clone().contract.address,
                current_swap_route.clone().token_in_denom,
            )?
            .amount;

        // Throw an Error if contract balance for the wanted denom is 0
        if balance_in_contract == Uint128::zero() {
            // TODO: Use InsufficientFundsForSwap instead, this has been removed after STRATEGIST_REWARDS state eval removal
            return Err(ContractError::InsufficientFunds {});
        }

        let pool_config = POOL_CONFIG.load(deps.storage)?;

        let part_1_amount = balance_in_contract.checked_div(Uint128::new(2)).unwrap();
        let part_2_amount = balance_in_contract
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
        .add_submessages(swap_msgs) // TODO: Adjust that to add_messages() if above doubt is resolved in favor of that
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
