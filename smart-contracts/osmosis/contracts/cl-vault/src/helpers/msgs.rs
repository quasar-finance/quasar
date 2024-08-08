use cosmwasm_std::{
    attr, to_json_binary, Addr, Attribute, BankMsg, Coin, CosmosMsg, Deps, Env, Uint128, WasmMsg,
};
use dex_router_osmosis::msg::ExecuteMsg as DexRouterExecuteMsg;
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin as OsmoCoin,
    osmosis::{
        concentratedliquidity::v1beta1::{MsgCollectIncentives, MsgCollectSpreadRewards},
        poolmanager::v1beta1::SwapAmountInRoute,
    },
};

use crate::{state::POSITION, vault::swap::SwapParams, ContractError};

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
pub fn swap_msg(
    sender: Addr,
    params: SwapParams,
    dex_router: Option<Addr>,
) -> Result<CosmosMsg, ContractError> {
    let pool_route = SwapAmountInRoute {
        pool_id: params.pool_id,
        token_out_denom: params.token_out_denom.to_string(),
    };

    if dex_router.is_none() {
        return Ok(osmosis_swap_exact_amount_in_msg(
            sender,
            pool_route,
            params.token_in_amount,
            &params.token_in_denom.to_string(),
            params.token_out_min_amount,
        ));
    }

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
    sender: Addr,
    pool_route: SwapAmountInRoute,
    token_in_amount: Uint128,
    token_in_denom: &String,
    token_out_min_amount: Uint128,
) -> CosmosMsg {
    osmosis_std::types::osmosis::poolmanager::v1beta1::MsgSwapExactAmountIn {
        sender: sender.to_string(),
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
