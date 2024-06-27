use cosmwasm_std::{
    attr, to_json_binary, Addr, Attribute, BankMsg, Coin, CosmosMsg, Deps, DepsMut, Env, Uint128,
    WasmMsg,
};
use dex_router_osmosis::msg::ExecuteMsg as DexRouterExecuteMsg;
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as OsmoCoin,
    osmosis::{
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
            attributes.push(attr("refund0_amount", refund0.amount));
            attributes.push(attr("refund0_denom", refund0.denom.as_str()));
            coins.push(refund0)
        }
    }
    if let Some(refund1) = refund1 {
        if refund1.amount > Uint128::zero() {
            attributes.push(attr("refund1_amount", refund1.amount));
            attributes.push(attr("refund1_denom", refund1.denom.as_str()));
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
pub fn swap_msg(deps: &DepsMut, env: &Env, params: SwapParams) -> Result<CosmosMsg, ContractError> {
    // let pool_config = POOL_CONFIG.load(deps.storage)?;
    let dex_router = DEX_ROUTER.may_load(deps.storage)?;

    // we will only ever have a route length of one, this will likely change once we start selecting different routes
    let pool_route = SwapAmountInRoute {
        pool_id: params.pool_id,
        token_out_denom: params.token_out_denom.to_string(),
    };

    // if we don't have a dex_router, we will always swap over the osmosis pool
    if dex_router.is_none() {
        return Ok(osmosis_swap_exact_amount_in_msg(
            env,
            pool_route,
            params.token_in_amount,
            &params.token_in_denom.to_string(),
            params.token_out_min_amount,
        ));
    }

    // we know we have a dex_router, so we can unwrap it and execute the swap
    cw_dex_execute_swap_operations_msg(
        dex_router.clone().unwrap(),
        params.forced_swap_route,
        params.token_in_denom.to_string(),
        params.token_in_amount,
        params.token_out_denom.to_string(),
        params.token_out_min_amount,
    )
}

fn osmosis_swap_exact_amount_in_msg(
    env: &Env,
    pool_route: SwapAmountInRoute,
    token_in_amount: Uint128,
    token_in_denom: &String,
    token_out_min_amount: Uint128,
) -> CosmosMsg {
    osmosis_std::types::osmosis::poolmanager::v1beta1::MsgSwapExactAmountIn {
        sender: env.contract.address.to_string(),
        routes: vec![pool_route],
        token_in: Some(OsmoCoin {
            denom: token_in_denom.to_string(),
            amount: token_in_amount.to_string(),
        }),
        token_out_min_amount: token_out_min_amount.to_string(),
    }
    .into()
}

fn cw_dex_execute_swap_operations_msg(
    dex_router_address: Addr,
    path: Option<Vec<SwapAmountInRoute>>,
    token_in_denom: String,
    token_in_amount: Uint128,
    token_out_denom: String,
    token_out_min_amount: Uint128,
) -> Result<CosmosMsg, ContractError> {
    let swap_msg: CosmosMsg = WasmMsg::Execute {
        contract_addr: dex_router_address.to_string(),
        msg: to_json_binary(&DexRouterExecuteMsg::Swap {
            path,
            out_denom: token_out_denom,
            minimum_receive: Some(token_out_min_amount),
        })?,
        funds: vec![Coin {
            denom: token_in_denom,
            amount: token_in_amount,
        }],
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
