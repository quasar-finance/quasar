use cosmwasm_std::{
    coin, to_json_binary, Addr, CheckedMultiplyFractionError, Coin, CosmosMsg, Decimal, DepsMut,
    Env, MessageInfo, Response, Uint128, WasmMsg,
};
use dex_router_osmosis::msg::ExecuteMsg as DexRouterExecuteMsg;
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;

use crate::{
    error::assert_swap_admin,
    helpers::getters::get_twap_price,
    msg::SwapOperation,
    state::{DEX_ROUTER, POOL_CONFIG, VAULT_CONFIG},
    ContractError,
};

pub fn execute_swap_non_vault_funds(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    swap_operations: Vec<SwapOperation>,
    twap_window_seconds: Option<u64>,
) -> Result<Response, ContractError> {
    assert_swap_admin(deps.storage, &info.sender)?;

    let pool_config = POOL_CONFIG.load(deps.storage)?;

    if swap_operations.is_empty() {
        return Err(ContractError::EmptySwapOperations {});
    }

    let mut swap_msgs: Vec<CosmosMsg> = vec![];

    for swap_operation in swap_operations {
        let token_in_denom = &swap_operation.token_in_denom;

        if token_in_denom == &pool_config.token0 || token_in_denom == &pool_config.token1 {
            return Err(ContractError::InvalidSwapAssets {});
        }

        let token_in_balance = deps
            .querier
            .query_balance(env.clone().contract.address, token_in_denom.clone())?
            .amount;
        if token_in_balance.is_zero() {
            return Err(ContractError::InsufficientFundsForSwap {
                balance: token_in_balance,
                needed: Uint128::new(2), // we need at least 2 udenom to be able swap into 2 denoms
            });
        }

        let token_in_amount = token_in_balance.checked_div(Uint128::new(2))?;

        swap_msgs.push(prepare_swap_msg(
            &deps,
            &env,
            coin(token_in_amount.into(), token_in_denom.clone()),
            pool_config.clone().token0,
            swap_operation.pool_id_base,
            swap_operation.forced_swap_route_base,
            twap_window_seconds,
        )?);

        swap_msgs.push(prepare_swap_msg(
            &deps,
            &env,
            coin(token_in_amount.into(), token_in_denom.clone()),
            pool_config.clone().token1,
            swap_operation.pool_id_quote,
            swap_operation.forced_swap_route_quote,
            twap_window_seconds,
        )?);
    }

    Ok(Response::new()
        .add_messages(swap_msgs)
        .add_attribute("method", "execute")
        .add_attribute("action", "swap_non_vault_funds"))
}

fn prepare_swap_msg(
    deps: &DepsMut,
    env: &Env,
    token_in: Coin,
    token_out_denom: String,
    pool_id: u64,
    forced_swap_route: Option<Vec<SwapAmountInRoute>>,
    twap_window_seconds: Option<u64>,
) -> Result<CosmosMsg, ContractError> {
    let vault_config = VAULT_CONFIG.load(deps.storage)?;
    let dex_router = DEX_ROUTER.may_load(deps.storage)?;

    let twap_price = get_twap_price(
        &deps.querier,
        env.block.time,
        twap_window_seconds.unwrap_or_default(),
        pool_id,
        token_in.denom.to_string(),
        token_out_denom.to_string(),
    )?;

    let token_out_min_amount =
        estimate_swap_min_out_amount(token_in.amount, twap_price, vault_config.swap_max_slippage)?;

    let swap_msg = swap_msg(
        env.clone().contract.address,
        pool_id,
        token_in,
        coin(token_out_min_amount.into(), token_out_denom),
        forced_swap_route,
        dex_router,
    )?;

    Ok(swap_msg)
}

pub fn estimate_swap_min_out_amount(
    in_amount: Uint128,
    price: Decimal,
    slippage_factor: Decimal,
) -> Result<Uint128, CheckedMultiplyFractionError> {
    in_amount
        .checked_mul_floor(price)?
        .checked_mul_floor(slippage_factor)
}

pub fn swap_msg(
    sender: Addr,
    pool_id: u64,
    token_in: Coin,
    min_receive: Coin,
    forced_swap_route: Option<Vec<SwapAmountInRoute>>,
    dex_router: Option<Addr>,
) -> Result<CosmosMsg, ContractError> {
    if let Some(dex_router) = dex_router {
        cw_dex_execute_swap_operations_msg(dex_router, forced_swap_route, token_in, min_receive)
    } else {
        let pool_route = SwapAmountInRoute {
            pool_id,
            token_out_denom: min_receive.denom,
        };
        Ok(osmosis_swap_exact_amount_in_msg(
            sender,
            pool_route,
            token_in,
            min_receive.amount,
        ))
    }
}

fn osmosis_swap_exact_amount_in_msg(
    sender: Addr,
    pool_route: SwapAmountInRoute,
    token_in: Coin,
    token_out_min_amount: Uint128,
) -> CosmosMsg {
    osmosis_std::types::osmosis::poolmanager::v1beta1::MsgSwapExactAmountIn {
        sender: sender.to_string(),
        routes: vec![pool_route],
        token_in: Some(token_in.into()),
        token_out_min_amount: token_out_min_amount.to_string(),
    }
    .into()
}

fn cw_dex_execute_swap_operations_msg(
    dex_router_address: Addr,
    path: Option<Vec<SwapAmountInRoute>>,
    token_in: Coin,
    min_receive: Coin,
) -> Result<CosmosMsg, ContractError> {
    let swap_msg: CosmosMsg = WasmMsg::Execute {
        contract_addr: dex_router_address.to_string(),
        msg: to_json_binary(&DexRouterExecuteMsg::Swap {
            path,
            out_denom: min_receive.denom,
            minimum_receive: Some(min_receive.amount),
        })?,
        funds: vec![token_in],
    }
    .into();

    Ok(swap_msg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::Uint128;

    #[test]
    fn test_estimate_swap_min_out_amount() {
        let in_amount = Uint128::from(50u64);
        let price = Decimal::percent(300);
        let max_slippage = Decimal::percent(50);

        let out_amount = estimate_swap_min_out_amount(in_amount, price, max_slippage).unwrap();
        assert_eq!(out_amount, Uint128::from(75u64));
    }
}
