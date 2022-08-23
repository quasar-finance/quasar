use std::env;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint64,
};
use cw2::set_contract_version;
use intergamm_bindings::msg::IntergammMsg;

use crate::error::ContractError;
use crate::msg::{AckTriggeredResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{State, ACKTRIGGERED, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:intergamm-bindings-test-2";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // set the ack triggered to false
    ACKTRIGGERED.save(deps.storage, &0);
    Ok(Response::default())
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
        } => execute_send_token(destination_local_zone_id, env),
        ExecuteMsg::RegisterInterchainAccount { connection_id } => execute_register_ica(connection_id, env),
        ExecuteMsg::JoinSinglePool {
            connection_id,
            pool_id,
            share_out_min_amount,
            token_in,
        } => execute_join_pool(connection_id, pool_id, share_out_min_amount, token_in, env),
        ExecuteMsg::AckTriggered {} => do_ibc_packet_ack(deps, env),
        ExecuteMsg::Deposit {} => execute_deposit(info),
    }
}

// TODO as of 23 august, there is a bug in the go implementation of send token
pub fn execute_send_token(
    destination_local_zone_id: String,
    env: Env,
) -> Result<Response<IntergammMsg>, ContractError> {
    Ok(Response::new()
        .add_message(IntergammMsg::SendToken {
            creator: env.contract.address.to_string(),
            destination_local_zone_id: destination_local_zone_id,
            sender: env.contract.address.to_string(),
            receiver: env.contract.address.to_string(),
            coin: Coin::new(100, "uqsr"),
        })
        .add_attribute("sending tokens", "100 uqsr to osmosis"))
}

// join pool requires us to have a pool on the remote chain
pub fn execute_join_pool(
    connection_id: String,
    pool_id: Uint64,
    share_out_min_amount: i64,
    token_in: Coin,
    env: Env,
) -> Result<Response<IntergammMsg>, ContractError> {
    Ok(
        Response::new().add_message(IntergammMsg::JoinSwapExternAmountIn {
            creator: env.contract.address.to_string(),
            connection_id,
            // timeout in 10 minutes
            timeout_timestamp: env.block.time.plus_seconds(600).nanos(),
            pool_id: pool_id.u64(),
            share_out_min_amount,
            token_in,
        }),
    )
}

pub fn execute_register_ica(
    connection_id: String,
    env: Env,
) -> Result<Response<IntergammMsg>, ContractError> {
    Ok(
        Response::new().add_message(IntergammMsg::RegisterInterchainAccount {
            creator: env.contract.address.to_string(),
            connection_id: connection_id,
        }),
    )
}

pub fn execute_deposit(info: MessageInfo) -> Result<Response<IntergammMsg>, ContractError> {
    let funds = cw_utils::must_pay(&info, "uqsr")?;
    // we dont do anything else with the funds since we solely use them for testing and don't need to deposit
    Ok(Response::new().add_attribute("deposit", funds))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AckTriggered {} => to_binary(&query_ack_triggered(deps)?),
    }
}

pub fn query_ack_triggered(deps: Deps) -> StdResult<AckTriggeredResponse> {
    let state = ACKTRIGGERED.load(deps.storage)?;
    Ok(AckTriggeredResponse { state })
}

pub fn do_ibc_packet_ack(
    deps: DepsMut,
    _env: Env,
) -> Result<Response<IntergammMsg>, ContractError> {
    let triggered = ACKTRIGGERED.load(deps.storage)?;
    ACKTRIGGERED.save(deps.storage, &(triggered + 1))?;
    Ok(Response::new().add_attribute("ack tiggered", (triggered + 1).to_string()))
}

#[cfg(test)]
mod tests {}
