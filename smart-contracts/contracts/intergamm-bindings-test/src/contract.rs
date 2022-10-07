use cosmwasm_std::{
    entry_point, to_binary, Binary, Coin, Deps, DepsMut, Env, IbcMsg, IbcTimeout, MessageInfo,
    Order, Reply, Response, StdError, StdResult, Uint64,
};
use cw2::set_contract_version;
use intergamm_bindings::helper::{
    check_callback_addr, create_intergamm_msg, handle_reply, set_callback_addr, ack,
};
use intergamm_bindings::msg::IntergammMsg;

use crate::error::ContractError;
use crate::msg::{AcksResponse, ExecuteMsg, InstantiateMsg, PendingAcksResponse, QueryMsg};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:intergamm-bindings-test";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    set_callback_addr(deps, &msg.callback_address)?;
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> StdResult<Response> {
    handle_reply(deps.storage, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<IntergammMsg>, ContractError> {
    match msg {
        ExecuteMsg::SendToken {
            destination_local_zone_id,
            receiver,
            coin
        } => execute_send_token(destination_local_zone_id, receiver, coin),
        ExecuteMsg::SendTokenIbc {
            channel_id,
            to_address,
            amount,
        } => execute_send_token_ibc(channel_id, to_address, amount, env),
        ExecuteMsg::RegisterIcaOnZone { zone_id } => {
            execute_register_ica_on_zone(zone_id, deps)
        }
        ExecuteMsg::JoinSwapExternAmountIn {
            connection_id,
            pool_id,
            share_out_min_amount,
            token_in,
        } => execute_join_swap_extern_amount_in(
            connection_id,
            pool_id,
            share_out_min_amount,
            token_in,
            deps,
            env,
        ),
        ExecuteMsg::TestIcaScenario {} => execute_test_scenario("registerIca".to_string()),
        ExecuteMsg::Ack {
            sequence_number,
            error,
            response,
        } => do_ibc_packet_ack(deps, env, info, sequence_number, error, response),
        ExecuteMsg::Deposit {} => execute_deposit(info),
        ExecuteMsg::JoinPool {
            connection_id,
            timeout_timestamp,
            pool_id,
            share_out_amount,
            token_in_maxs,
        } => execute_join_pool(
            connection_id,
            timeout_timestamp,
            pool_id,
            share_out_amount,
            token_in_maxs,
            deps,
        ),
        ExecuteMsg::ExitPool {
            connection_id,
            timeout_timestamp,
            pool_id,
            share_in_amount,
            token_out_mins,
        } => execute_exit_pool(
            connection_id,
            timeout_timestamp,
            pool_id,
            share_in_amount,
            token_out_mins,
            deps,
        ),
        ExecuteMsg::LockTokens {
            connection_id,
            timeout_timestamp,
            duration,
            coins,
        } => execute_lock_tokens(connection_id, timeout_timestamp, duration, coins, deps),
        ExecuteMsg::ExitSwapExternAmountOut {
            connection_id,
            timeout_timestamp,
            pool_id,
            share_in_amount,
            token_out_mins,
        } => execute_exit_swap_extern_amount_out(
            connection_id,
            timeout_timestamp,
            pool_id,
            share_in_amount,
            token_out_mins,
            deps,
        ),
        ExecuteMsg::BeginUnlocking {
            connection_id,
            timeout_timestamp,
            id,
            coins,
        } => execute_begin_unlocking(connection_id, timeout_timestamp, id, coins, deps),
    }
}

pub fn execute_exit_pool(
    connection_id: String,
    timeout_timestamp: Uint64,
    pool_id: Uint64,
    share_in_amount: i64,
    token_out_mins: Vec<Coin>,
    deps: DepsMut,
) -> Result<Response<IntergammMsg>, ContractError> {
    let msg = IntergammMsg::ExitPool {
        connection_id,
        timeout_timestamp: timeout_timestamp.u64(),
        pool_id: pool_id.u64(),
        share_in_amount,
        token_out_mins,
    };
    create_intergamm_msg(deps.storage, msg).map_err(ContractError::Std)
}

pub fn execute_join_pool(
    connection_id: String,
    timeout_timestamp: Uint64,
    pool_id: Uint64,
    share_out_amount: i64,
    token_in_maxs: Vec<Coin>,
    deps: DepsMut,
) -> Result<Response<IntergammMsg>, ContractError> {
    let msg = IntergammMsg::JoinPool {
        connection_id,
        timeout_timestamp: timeout_timestamp.u64(),
        pool_id: pool_id.u64(),
        share_out_amount,
        token_in_maxs,
    };
    create_intergamm_msg(deps.storage, msg).map_err(ContractError::Std)
}

pub fn execute_lock_tokens(
    connection_id: String,
    timeout_timestamp: Uint64,
    duration: Uint64,
    coins: Vec<Coin>,
    deps: DepsMut,
) -> Result<Response<IntergammMsg>, ContractError> {
    let msg = IntergammMsg::LockTokens {
        connection_id,
        timeout_timestamp: timeout_timestamp.u64(),
        duration: duration.u64(),
        coins,
    };
    create_intergamm_msg(deps.storage, msg).map_err(ContractError::Std)
}

pub fn execute_begin_unlocking(
    connection_id: String,
    timeout_timestamp: Uint64,
    id: Uint64,
    coins: Vec<Coin>,
    deps: DepsMut,
) -> Result<Response<IntergammMsg>, ContractError> {
    let msg = IntergammMsg::BeginUnlocking {
        connection_id,
        timeout_timestamp: timeout_timestamp.u64(),
        id: id.u64(),
        coins,
    };
    create_intergamm_msg(deps.storage, msg).map_err(ContractError::Std)
}

pub fn execute_exit_swap_extern_amount_out(
    connection_id: String,
    timeout_timestamp: Uint64,
    pool_id: Uint64,
    share_in_amount: i64,
    token_out_mins: Coin,
    deps: DepsMut,
) -> Result<Response<IntergammMsg>, ContractError> {
    let msg = IntergammMsg::ExitSwapExternAmountOut {
        connection_id,
        timeout_timestamp: timeout_timestamp.u64(),
        pool_id: pool_id.u64(),
        share_in_amount,
        token_out_mins,
    };
    create_intergamm_msg(deps.storage, msg).map_err(ContractError::Std)
}

pub fn execute_send_token(
    destination_local_zone_id: String,
    receiver: String,
    coin: Coin,
) -> Result<Response<IntergammMsg>, ContractError> {
    // receiver is an address on a different chain, so we can't parse it.
    Ok(Response::new()
        .add_attribute("send_tokens", format!("{} {} to {}", coin.amount, coin.denom, destination_local_zone_id))
        .add_message(IntergammMsg::SendToken {
            destination_local_zone_id,
            receiver: receiver,
            coin: coin,
        }))
}

pub fn execute_send_token_ibc(
    channel_id: String,
    to_address: String,
    amount: Coin,
    env: Env,
) -> Result<Response<IntergammMsg>, ContractError> {
    // timeout in 600 seconds after current block timestamp
    let timeout = IbcTimeout::with_timestamp(env.block.time.plus_seconds(600));
    Ok(Response::new().add_message(IbcMsg::Transfer {
        channel_id,
        to_address,
        amount,
        timeout,
    }))
}

pub fn execute_test_scenario(scenario: String) -> Result<Response<IntergammMsg>, ContractError> {
    Ok(Response::new().add_message(IntergammMsg::TestScenario { scenario }))
}

pub fn execute_register_ica_on_zone(
    zone_id: String,
    deps: DepsMut,
) -> Result<Response<IntergammMsg>, ContractError> {
    let msg = IntergammMsg::RegisterIcaOnZone { zone_id };
    create_intergamm_msg(deps.storage, msg).map_err(ContractError::Std)
}

// join pool requires us to have a pool on the remote chain and funds in the interchain account of this contract
pub fn execute_join_swap_extern_amount_in(
    connection_id: String,
    pool_id: Uint64,
    share_out_min_amount: i64,
    token_in: Coin,
    deps: DepsMut,
    env: Env,
) -> Result<Response<IntergammMsg>, ContractError> {
    let msg = IntergammMsg::JoinSwapExternAmountIn {
        connection_id,
        // timeout in 10 minutes
        timeout_timestamp: env.block.time.plus_seconds(600).nanos(),
        pool_id: pool_id.u64(),
        share_out_min_amount,
        token_in,
    };
    create_intergamm_msg(deps.storage, msg).map_err(ContractError::Std)
}

pub fn execute_deposit(info: MessageInfo) -> Result<Response<IntergammMsg>, ContractError> {
    let funds = cw_utils::one_coin(&info)?;
    if funds.denom != "uqsr" && funds.denom != "stake" {
        return Err(ContractError::PaymentError(
            cw_utils::PaymentError::MissingDenom("uqsr/stake".into()),
        ));
    }
    // we dont do anything else with the funds since we solely use them for testing and don't need to deposit
    Ok(Response::new()
        .add_attribute("deposit_amount", funds.amount)
        .add_attribute("deposit_denom", funds.denom))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Acks {} => to_binary(&query_acks(deps)?),
        QueryMsg::PendingAcks {} => to_binary(&query_pending_acks(deps)?),
    }
}

pub fn query_acks(deps: Deps) -> StdResult<AcksResponse> {
    let acks: Result<Vec<(u64, intergamm_bindings::msg::AckValue)>, StdError> = intergamm_bindings::state::ACKS
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    Ok(AcksResponse { acks: acks? })
}

pub fn query_pending_acks(deps: Deps) -> StdResult<PendingAcksResponse> {
    let pending: Result<Vec<(u64, IntergammMsg)>, StdError> = intergamm_bindings::state::PENDINGACKS
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    Ok(PendingAcksResponse { pending: pending? })
}

pub fn do_ibc_packet_ack(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    sequence: u64,
    error: Option<String>,
    response: Option<intergamm_bindings::msg::AckResponse>,
) -> Result<Response<IntergammMsg>, ContractError> {
    check_callback_addr(deps.as_ref(), info.sender)?;
    ack(deps, sequence, &error, &response)?;
    // Insert any further neede logic to handle acks here
    Ok(Response::new()
        .add_attribute("error", error.unwrap_or_else(|| "none".into()))
        .add_attribute("response", format!("{:?}", response)))
}

#[cfg(test)]
mod tests {}
