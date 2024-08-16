use cosmwasm_std::{
    coin, to_json_binary, Addr, CheckedMultiplyFractionError, Coin, CosmosMsg, Decimal, DepsMut,
    MessageInfo, Response, Uint128, WasmMsg,
};
use dex_router_osmosis::msg::ExecuteMsg as DexRouterExecuteMsg;
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;

use crate::{
    error::assert_range_admin,
    msg::SwapOperation,
    state::{DEX_ROUTER, POOL_CONFIG, VAULT_CONFIG},
    ContractError,
};

pub fn execute_swap_non_vault_funds(
    deps: DepsMut,
    contract_address: Addr,
    info: MessageInfo,
    swap_operations: Vec<SwapOperation>,
) -> Result<Response, ContractError> {
    assert_range_admin(deps.storage, &info.sender)?;

    let vault_config = VAULT_CONFIG.load(deps.storage)?;
    let pool_config = POOL_CONFIG.load(deps.storage)?;

    if swap_operations.is_empty() {
        return Err(ContractError::EmptySwapOperations {});
    }

    let mut swap_msgs: Vec<CosmosMsg> = vec![];

    for swap_operation in swap_operations {
        let token_in_denom = swap_operation.token_in_denom.clone();
        let pool_token_0 = pool_config.token0.clone();
        let pool_token_1 = pool_config.token1.clone();

        if token_in_denom == pool_token_0 || token_in_denom == pool_token_1 {
            return Err(ContractError::InvalidSwapAssets {});
        }

        let balance_in_contract = deps
            .querier
            .query_balance(
                contract_address.clone(),
                swap_operation.clone().token_in_denom,
            )?
            .amount;

        if balance_in_contract == Uint128::zero() {
            return Err(ContractError::InsufficientFunds {});
        }

        let part_0_amount = balance_in_contract.checked_div(Uint128::new(2))?;
        let part_1_amount = balance_in_contract
            .checked_add(Uint128::new(1))?
            .checked_div(Uint128::new(2))?;

        let token_out_min_amount_0 =
            part_0_amount.checked_mul_floor(vault_config.swap_max_slippage)?;
        let token_out_min_amount_1 =
            part_1_amount.checked_mul_floor(vault_config.swap_max_slippage)?;

        let dex_router = DEX_ROUTER.may_load(deps.storage)?;
        swap_msgs.push(swap_msg(
            contract_address.clone(),
            swap_operation.pool_id_0,
            coin(part_0_amount.into(), token_in_denom.clone()),
            coin(token_out_min_amount_0.into(), pool_token_0),
            swap_operation.forced_swap_route_token_0,
            dex_router.clone(),
        )?);
        swap_msgs.push(swap_msg(
            contract_address.clone(),
            swap_operation.pool_id_1,
            coin(part_1_amount.into(), token_in_denom.clone()),
            coin(token_out_min_amount_1.into(), pool_token_1),
            swap_operation.forced_swap_route_token_1,
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
