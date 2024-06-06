use cosmwasm_schema::cw_serde;
use cosmwasm_std::Order;
use cosmwasm_std::{
    to_json_binary, DepsMut, Env, MessageInfo, Response, SubMsg, SubMsgResult, Uint128,
};
use cw_dex_router::operations::SwapOperationsListUnchecked;
use osmosis_std::cosmwasm_to_proto_coins;
use osmosis_std::types::cosmos::bank::v1beta1::{Input, MsgMultiSend, Output};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::ConcentratedliquidityQuerier;
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::MsgCreatePositionResponse;

use crate::helpers::{get_unused_balances, must_pay_one_or_two_from_balance};
use crate::msg::{ExecuteMsg, MergePositionMsg};
use crate::reply::Replies;
#[allow(deprecated)]
use crate::state::USER_REWARDS;
use crate::state::{MigrationStatus, MIGRATION_STATUS, POOL_CONFIG, POSITION};
use crate::vault::concentrated_liquidity::create_position;
use crate::ContractError;

use super::helpers::CoinList;

#[cw_serde]
pub struct SwapAsset {
    pub token_in_denom: String,
    pub pool_id_0: u64, // the osmosis pool_id as mandatory to have at least the chance to swap on CL pools
    pub pool_id_1: u64, // the osmosis pool_id as mandatory to have at least the chance to swap on CL pools
    pub recommended_swap_route_token_0: Option<SwapOperationsListUnchecked>,
    pub recommended_swap_route_token_1: Option<SwapOperationsListUnchecked>,
}

pub fn execute_autocompound(
    deps: DepsMut,
    env: &Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    let position_state = POSITION.load(deps.storage)?;

    // If the position claim after timestamp is not reached yet, return an error
    if position_state.claim_after.is_some()
        && position_state.claim_after.unwrap() <= env.block.time.seconds()
    {
        return Err(ContractError::ClaimAfterNotExpired {});
    }

    let position = ConcentratedliquidityQuerier::new(&deps.querier)
        .position_by_id(position_state.position_id)?
        .position
        .ok_or(ContractError::PositionNotFound)?
        .position
        .ok_or(ContractError::PositionNotFound)?;

    let balance = get_unused_balances(&deps.querier, env)?;
    let pool = POOL_CONFIG.load(deps.storage)?;

    let (token0, token1) =
        must_pay_one_or_two_from_balance(balance.coins(), (pool.token0, pool.token1))?;

    // Create coins_to_send with no zero amounts
    let mut coins_to_send = vec![];
    if !token0.amount.is_zero() {
        coins_to_send.push(token0.clone());
    }
    if !token1.amount.is_zero() {
        coins_to_send.push(token1.clone());
    }

    let create_position_msg = create_position(
        deps,
        env,
        position.lower_tick,
        position.upper_tick,
        coins_to_send,
        Uint128::zero(),
        Uint128::zero(),
    )?;

    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(
            create_position_msg,
            Replies::Autocompound as u64,
        ))
        .add_attribute("method", "execute")
        .add_attribute("action", "autocompound")
        .add_attribute("lower_tick", format!("{:?}", position.lower_tick))
        .add_attribute("upper_tick", format!("{:?}", position.upper_tick))
        .add_attribute("token0", format!("{:?}", token0.clone()))
        .add_attribute("token1", format!("{:?}", token1.clone())))
}

pub fn handle_autocompound_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> Result<Response, ContractError> {
    let create_position_message: MsgCreatePositionResponse = data.try_into()?;

    // set claim after
    let position_id = (POSITION.load(deps.storage)?).position_id;
    // call merge
    let merge_msg =
        ExecuteMsg::VaultExtension(crate::msg::ExtensionExecuteMsg::Merge(MergePositionMsg {
            position_ids: vec![position_id, create_position_message.position_id],
        }));

    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(
            cosmwasm_std::WasmMsg::Execute {
                contract_addr: env.contract.address.to_string(),
                msg: to_json_binary(&merge_msg)?,
                funds: vec![],
            },
            Replies::Merge.into(),
        ))
        .add_attribute("method", "reply")
        .add_attribute("action", "handle_autocompound")
        .add_attribute(
            "position_ids",
            format!("{:?}", vec![create_position_message.position_id]),
        ))
}

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
    #[allow(deprecated)]
    for item in USER_REWARDS
        .range(deps.storage, None, None, Order::Ascending)
        .take(amount_of_users.u128() as usize)
    {
        let (address, rewards) = item?;

        // We always push the address in order to remove it later
        addresses.push(address.clone());
        // If there are no rewards, we skip the address or we will get invalid_coins error
        // This is because USER_REWARDS is holding 0 amount coins. rewards.coins() only returns a list of coins with non-zero amounts, which it could be empty
        if rewards.coins().is_empty() {
            continue;
        }
        outputs.push(Output {
            address: address.to_string(),
            coins: cosmwasm_to_proto_coins(rewards.coins().iter().cloned()),
        });
        total_amount.add(rewards)?;
    }

    // Remove processed rewards in a separate iteration.
    #[allow(deprecated)]
    for addr in addresses {
        USER_REWARDS.remove(deps.storage, addr);
    }

    // Check if this is the last execution.
    #[allow(deprecated)]
    let is_last_execution = USER_REWARDS
        .range(deps.storage, None, None, Order::Ascending)
        .next()
        .is_none();
    if is_last_execution {
        migration_status = MigrationStatus::Closed;
        MIGRATION_STATUS.save(deps.storage, &migration_status)?;
    }

    let mut response = Response::new();
    // Only if there are rewards append the send_message
    if !total_amount.is_empty() {
        let send_message = MsgMultiSend {
            inputs: vec![Input {
                address: env.contract.address.to_string(),
                coins: cosmwasm_to_proto_coins(total_amount.coins().iter().cloned()),
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
