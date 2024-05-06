use cosmwasm_std::Order;
use cosmwasm_std::{
    to_json_binary, DepsMut, Env, MessageInfo, Response, SubMsg, SubMsgResult, Uint128,
};
use osmosis_std::types::cosmos::bank::v1beta1::{Input, MsgMultiSend, Output};
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::ConcentratedliquidityQuerier;
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::MsgCreatePositionResponse;

use crate::error::ContractResult;
use crate::helpers::{get_unused_balances, must_pay_one_or_two_from_balance};
use crate::msg::{ExecuteMsg, MergePositionMsg};
use crate::reply::Replies;
use crate::rewards::CoinList;
use crate::state::USER_REWARDS;
use crate::state::{MigrationStatus, MIGRATION_STATUS, POOL_CONFIG, POSITION};
use crate::vault::concentrated_liquidity::create_position;
use crate::ContractError;

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

        addresses.push(address.clone());
        outputs.push(Output {
            address: address.to_string(),
            coins: rewards.osmo_coin_from_coin_list(),
        });
        total_amount.add(rewards)?;
    }

    // Remove processed rewards in a separate iteration.
    for addr in addresses {
        USER_REWARDS.remove(deps.storage, addr);
    }

    // Check if this is the last execution.
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

/// Execute the autocompound process, creating a new position with unused balances.
///
/// # Arguments
///
/// * `deps` - Dependencies for interacting with the contract.
/// * `env` - Environment for fetching contract address.
/// * `_info` - Message information (not used).
///
/// # Errors
///
/// Returns a `ContractError` if the operation fails.
///
/// # Returns
///
/// Returns a `Response` containing the result of the autocompound operation.
pub fn execute_autocompound(
    deps: DepsMut,
    env: &Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    // TODO: Validate claim_after timestamp

    let position_id = (POSITION.load(deps.storage)?).position_id;
    let position = ConcentratedliquidityQuerier::new(&deps.querier)
        .position_by_id(position_id)?
        .position
        .ok_or(ContractError::PositionNotFound)?
        .position
        .ok_or(ContractError::PositionNotFound)?;

    let balance = get_unused_balances(&deps.querier, &env).unwrap();
    let pool = POOL_CONFIG.load(deps.storage)?;

    // TODO: We should swap() here

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
        &env,
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

/// Handle the reply from the autocompound operation and then calling merge position
/// on the newly created position.
///
/// # Arguments
///
/// * `deps` - Dependencies for interacting with the contract.
/// * `env` - Environment for fetching contract address.
/// * `data` - Result of the autocompound operation.
///
/// # Errors
///
/// Returns a `ContractError` if the operation fails.
///
/// # Returns
///
/// Returns a `Response` containing the result of the merge operation.
pub fn handle_autocompound_reply(
    deps: DepsMut,
    env: Env,
    data: SubMsgResult,
) -> ContractResult<Response> {
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
        .add_attribute("action", "handle_autocompound_reply")
        .add_attribute(
            "position_ids",
            format!("{:?}", vec![create_position_message.position_id]),
        ))
}