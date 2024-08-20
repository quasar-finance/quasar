use cosmwasm_std::{
    coin, to_json_binary, Addr, CheckedMultiplyFractionError, Coin, CosmosMsg, Decimal, DepsMut,
    Env, Response, Uint128, WasmMsg,
};
use dex_router_osmosis::msg::ExecuteMsg as DexRouterExecuteMsg;
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;

use crate::{
    helpers::getters::{get_position_balance, get_twap_price},
    msg::SwapOperation,
    state::{DEX_ROUTER, POOL_CONFIG, VAULT_CONFIG},
    ContractError,
};

pub fn execute_swap_non_vault_funds(
    deps: DepsMut,
    env: Env,
    swap_operations: Vec<SwapOperation>,
    twap_window_seconds: Option<u64>,
) -> Result<Response, ContractError> {
    let vault_config = VAULT_CONFIG.load(deps.storage)?;
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

        let balance_in_contract = deps
            .querier
            .query_balance(env.clone().contract.address, token_in_denom.clone())?
            .amount;
        if balance_in_contract.is_zero() {
            return Err(ContractError::InsufficientFundsForSwap {
                balance: balance_in_contract,
                needed: Uint128::new(1),
            });
        }

        // Get the current position balance ratio to compute the amount of external funds we want to swap into either token0 or token1 from the vault's pool
        let position_balance = get_position_balance(deps.storage, &deps.querier)?;
        let to_token0_amount = Uint128::from(
            (balance_in_contract.u128() as f64
                * position_balance.0.to_string().parse::<f64>().unwrap()) as u128,
        );
        let to_token1_amount = Uint128::from(
            (balance_in_contract.u128() as f64
                * position_balance.1.to_string().parse::<f64>().unwrap()) as u128,
        );

        let dex_router = DEX_ROUTER.may_load(deps.storage)?;

        let twap_price_base = get_twap_price(
            &deps.querier,
            env.block.time,
            twap_window_seconds.unwrap_or_default(), // default to 0 if not provided
            swap_operation.pool_id_base,
            token_in_denom.to_string(),
            pool_config.clone().token0,
        )?;
        let token_out_min_amount_base = estimate_swap_min_out_amount(
            to_token0_amount,
            twap_price_base,
            vault_config.swap_max_slippage,
        )?;
        swap_msgs.push(swap_msg(
            env.clone().contract.address,
            swap_operation.pool_id_base,
            coin(to_token0_amount.into(), token_in_denom.clone()),
            coin(token_out_min_amount_base.into(), pool_config.clone().token0),
            swap_operation.forced_swap_route_base,
            dex_router.clone(),
        )?);

        let twap_price_quote = get_twap_price(
            &deps.querier,
            env.block.time,
            twap_window_seconds.unwrap_or_default(), // default to 0 if not provided
            swap_operation.pool_id_quote,
            token_in_denom.to_string(),
            pool_config.clone().token1,
        )?;
        let token_out_min_amount_quote = estimate_swap_min_out_amount(
            to_token1_amount,
            twap_price_quote,
            vault_config.swap_max_slippage,
        )?;
        swap_msgs.push(swap_msg(
            env.clone().contract.address,
            swap_operation.pool_id_quote,
            coin(to_token1_amount.into(), token_in_denom.clone()),
            coin(
                token_out_min_amount_quote.into(),
                pool_config.clone().token1,
            ),
            swap_operation.forced_swap_route_quote,
            dex_router,
        )?);
    }

    Ok(Response::new()
        .add_messages(swap_msgs)
        .add_attribute("method", "execute")
        .add_attribute("action", "swap_non_vault_funds"))
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
