use cosmwasm_std::{
    attr, to_json_binary, Addr, Attribute, BankMsg, Coin, CosmosMsg, Deps, DepsMut, Env, Uint128,
    WasmMsg,
};
use dex_router_osmosis::msg::ExecuteMsg as DexRouterExecuteMsg;
use osmosis_std::{
    cosmwasm_to_proto_coins,
    types::osmosis::{
        concentratedliquidity::v1beta1::{MsgCollectIncentives, MsgCollectSpreadRewards},
        poolmanager::v1beta1::SwapAmountInRoute,
    },
};

use crate::{
    state::{DEX_ROUTER, POSITION},
    vault::swap::SwapParams,
    ContractError,
};

// Bank

/// Generate a bank message and attributes for refunding tokens to a recipient.
pub fn refund_bank_msg(
    receiver: Addr,
    refund0: Option<Coin>,
    refund1: Option<Coin>,
) -> Result<Option<(BankMsg, Vec<Attribute>)>, ContractError> {
    let mut attributes: Vec<Attribute> = vec![];
    let mut coins: Vec<Coin> = vec![];

    if let Some(refund0) = refund0 {
        if refund0.amount > Uint128::zero() {
            attributes.push(attr("refund0", refund0.amount));
            coins.push(refund0)
        }
    }
    if let Some(refund1) = refund1 {
        if refund1.amount > Uint128::zero() {
            attributes.push(attr("refund1", refund1.amount));
            coins.push(refund1)
        }
    }
    let result: Option<(BankMsg, Vec<Attribute>)> = if !coins.is_empty() {
        Some((
            BankMsg::Send {
                to_address: receiver.to_string(),
                amount: coins,
            },
            attributes,
        ))
    } else {
        None
    };
    Ok(result)
}

/// Swaps

/// swap will always swap over the CL pool. In the future we may expand the
/// feature such that it chooses best swaps over all routes
pub fn swap_msg(
    deps: &DepsMut,
    contract_address: Addr,
    params: SwapParams,
) -> Result<CosmosMsg, ContractError> {
    // let pool_config = POOL_CONFIG.load(deps.storage)?;
    let dex_router = DEX_ROUTER.may_load(deps.storage)?;

    // we will only ever have a route length of one, this will likely change once we start selecting different routes
    let pool_route = SwapAmountInRoute {
        pool_id: params.pool_id,
        token_out_denom: params.min_token_out.denom.to_string(),
    };

    // if we don't have a dex_router, we will always swap over the osmosis pool
    if dex_router.is_none() {
        return Ok(osmosis_swap_exact_amount_in_msg(
            contract_address,
            pool_route,
            params.token_in,
            params.min_token_out.amount,
        ));
    }

    // we know we have a dex_router, so we can unwrap it and execute the swap
    cw_dex_execute_swap_operations_msg(
        dex_router.clone().unwrap(),
        params.forced_swap_route,
        params.token_in,
        params.min_token_out,
    )
}

fn osmosis_swap_exact_amount_in_msg(
    contract_address: Addr,
    pool_route: SwapAmountInRoute,
    token_in: Coin,
    token_out_min_amount: Uint128,
) -> CosmosMsg {
    osmosis_std::types::osmosis::poolmanager::v1beta1::MsgSwapExactAmountIn {
        sender: contract_address.to_string(),
        routes: vec![pool_route],
        token_in: cosmwasm_to_proto_coins([token_in]).first().cloned(),
        token_out_min_amount: token_out_min_amount.to_string(),
    }
    .into()
}

fn cw_dex_execute_swap_operations_msg(
    dex_router_address: Addr,
    path: Option<Vec<SwapAmountInRoute>>,
    token_in: Coin,
    min_token_out: Coin,
) -> Result<CosmosMsg, ContractError> {
    let swap_msg: CosmosMsg = WasmMsg::Execute {
        contract_addr: dex_router_address.to_string(),
        msg: to_json_binary(&DexRouterExecuteMsg::Swap {
            path,
            out_denom: min_token_out.denom,
            minimum_receive: Some(min_token_out.amount),
        })?,
        funds: vec![token_in],
    }
    .into();

    Ok(swap_msg)
}

/// Collect Incentives

pub fn collect_incentives_msg(deps: Deps, env: Env) -> Result<MsgCollectIncentives, ContractError> {
    let position = POSITION.load(deps.storage)?;
    Ok(MsgCollectIncentives {
        position_ids: vec![position.position_id],
        sender: env.contract.address.into(),
    })
}

pub fn collect_spread_rewards_msg(
    deps: Deps,
    env: Env,
) -> Result<MsgCollectSpreadRewards, ContractError> {
    let position = POSITION.load(deps.storage)?;
    Ok(MsgCollectSpreadRewards {
        position_ids: vec![position.position_id],
        sender: env.contract.address.into(),
    })
}
