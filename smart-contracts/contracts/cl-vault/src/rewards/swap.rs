use apollo_cw_asset::AssetInfo;
use cosmwasm_std::{
    to_json_binary, Addr, Coin, CosmosMsg, DepsMut, Env, MessageInfo, QuerierWrapper, Response,
    Uint128, WasmMsg,
};
use cw_dex_router::{
    msg::{BestPathForPairResponse, ExecuteMsg as ApolloExecuteMsg, QueryMsg as ApolloQueryMsg},
    operations::SwapOperationsListUnchecked,
};

use crate::{helpers::assert_auto_compound_admin, state::POOL_CONFIG};
use crate::{msg::SwapAsset, state::VAULT_CONFIG, ContractError};

pub fn execute_swap_non_vault_funds(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    force_swap_route: bool,
    swap_routes: Vec<SwapAsset>,
) -> Result<Response, ContractError> {
    // auto compound admin as the purpose of swaps are mainly around autocompound non-vault assets into assets that can be actually compounded.
    assert_auto_compound_admin(deps.storage, &info.sender)?;

    let vault_config = VAULT_CONFIG.load(deps.storage)?;
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    if swap_routes.is_empty() {
        return Err(ContractError::EmptyCompoundAssetList {});
    }

    let mut swap_msgs: Vec<CosmosMsg> = vec![];

    for current_swap_route in swap_routes {
        // Assert that no BASE_DENOM / QUOTE_DENOM is trying to be swapped as token_in
        if current_swap_route.token_in_denom == pool_config.token0
            || current_swap_route.token_in_denom == pool_config.token1
        {
            return Err(ContractError::InvalidSwapAssets {});
        }

        // Throw an Error if contract balance for the wanted denom is 0
        let balance_in_contract = deps
            .querier
            .query_balance(
                env.clone().contract.address,
                current_swap_route.clone().token_in_denom,
            )?
            .amount;
        if balance_in_contract == Uint128::zero() {
            // TODO: Use InsufficientFundsForSwap instead, this has been removed after STRATEGIST_REWARDS state eval removal
            return Err(ContractError::InsufficientFunds {});
        }

        let part_1_amount = balance_in_contract.checked_div(Uint128::new(2)).unwrap();
        let part_2_amount = balance_in_contract
            .checked_add(Uint128::new(1))
            .unwrap()
            .checked_div(Uint128::new(2))
            .unwrap();

        swap_msgs.push(generate_swap_message(
            deps.querier,
            &vault_config.dex_router,
            &current_swap_route.recommended_swap_route_token_0,
            &current_swap_route.token_in_denom,
            part_1_amount,
            &pool_config.token0,
            force_swap_route,
        )?);
        swap_msgs.push(generate_swap_message(
            deps.querier,
            &vault_config.dex_router,
            &current_swap_route.recommended_swap_route_token_1,
            &current_swap_route.token_in_denom,
            part_2_amount,
            &pool_config.token1,
            force_swap_route,
        )?);
    }

    Ok(Response::new()
        .add_messages(swap_msgs)
        .add_attribute("method", "execute")
        .add_attribute("action", "auto_compund_swap"))
}

fn generate_swap_message(
    querier: QuerierWrapper,
    dex_router_addr: &Addr,
    current_swap_route: &Option<SwapOperationsListUnchecked>,
    token_in_denom: &String,
    token_in_amount: Uint128,
    token_out_denom: &String,
    force_swap_route: bool,
) -> Result<CosmosMsg, ContractError> {
    let offer_asset = AssetInfo::Native(token_in_denom.to_string());
    let ask_asset = AssetInfo::Native(token_out_denom.to_string());

    let recommended_out: Uint128 = match current_swap_route.clone() {
        Some(operations) => querier.query_wasm_smart(
            dex_router_addr.to_string(),
            &ApolloQueryMsg::SimulateSwapOperations {
                offer_amount: token_in_amount,
                operations,
            },
        )?,
        None => 0u128.into(),
    };
    let best_path: Option<BestPathForPairResponse> = querier.query_wasm_smart(
        dex_router_addr.to_string(),
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
    let swap_msg = get_execute_swap_operations_msg(
        dex_router_addr,
        route,
        Uint128::zero(),
        &token_in_denom,
        token_in_amount,
    );

    swap_msg
}

fn get_execute_swap_operations_msg(
    dex_router_address: &Addr,
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
