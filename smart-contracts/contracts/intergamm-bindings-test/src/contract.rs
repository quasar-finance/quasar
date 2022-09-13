use cosmwasm_std::{
    entry_point, to_binary, Binary, Coin, Deps, DepsMut, Env, IbcMsg, IbcTimeout, MessageInfo,
    Order, Reply, Response, StdError, StdResult, SubMsg, Uint64,
};
use cw2::set_contract_version;
use intergamm_bindings::helper::create_intergamm_msg;
use intergamm_bindings::msg::IntergammMsg;

use crate::error::ContractError;
use crate::msg::{AcksResponse, ExecuteMsg, InstantiateMsg, PendingAcksResponse, QueryMsg};
use crate::state::{ACKS, PENDINGACKS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:intergamm-bindings-test";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}

#[entry_point]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    intergamm_bindings::helper::handle_reply(deps.storage, msg, PENDINGACKS)
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
        } => {
            return execute_send_token(destination_local_zone_id, env);
        }
        ExecuteMsg::SendTokenIbc {
            channel_id,
            to_address,
            amount,
        } => {
            return execute_send_token_ibc(channel_id, to_address, amount, env);
        }
        ExecuteMsg::RegisterInterchainAccount { connection_id } => {
            return execute_register_ica(connection_id, deps, env);
        }
        ExecuteMsg::JoinSinglePool {
            connection_id,
            pool_id,
            share_out_min_amount,
            token_in,
        } => {
            return execute_join_pool(
                connection_id,
                pool_id,
                share_out_min_amount,
                token_in,
                deps,
                env,
            );
        }
        ExecuteMsg::TestIcaScenario {} => {
            return execute_test_scenario(env);
        }
        ExecuteMsg::Ack {
            sequence_number,
            error,
            response,
        } => {
            return do_ibc_packet_ack(deps, env, sequence_number, error, response);
        }
        ExecuteMsg::Deposit {} => {
            return execute_deposit(info);
        }
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

pub fn execute_test_scenario(env: Env) -> Result<Response<IntergammMsg>, ContractError> {
    Ok(Response::new().add_message(IntergammMsg::TestScenario {
        creator: env.contract.address.to_string(),
        scenario: "registerIca".to_string(),
    }))
}

pub fn execute_register_ica(
    connection_id: String,
    deps: DepsMut,
    env: Env,
) -> Result<Response<IntergammMsg>, ContractError> {
    let msg = IntergammMsg::RegisterInterchainAccount {
        creator: env.contract.address.to_string(),
        connection_id: connection_id,
    };
    create_intergamm_msg(deps.storage, msg).map_err(|e| ContractError::Std(e))
}

// join pool requires us to have a pool on the remote chain and funds in the interchain account of this contract
pub fn execute_join_pool(
    connection_id: String,
    pool_id: Uint64,
    share_out_min_amount: i64,
    token_in: Coin,
    deps: DepsMut,
    env: Env,
) -> Result<Response<IntergammMsg>, ContractError> {
    let msg = IntergammMsg::JoinSwapExternAmountIn {
        creator: env.contract.address.to_string(),
        connection_id,
        // timeout in 10 minutes
        timeout_timestamp: env.block.time.plus_seconds(600).nanos(),
        pool_id: pool_id.u64(),
        share_out_min_amount,
        token_in,
    };
    create_intergamm_msg(deps.storage, msg).map_err(|e| ContractError::Std(e))
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
    let acks: Result<Vec<(u64, intergamm_bindings::msg::AckValue)>, StdError> = ACKS
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    Ok(AcksResponse { acks: acks? })
}

pub fn query_pending_acks(deps: Deps) -> StdResult<PendingAcksResponse> {
    let pending: Result<Vec<(u64, IntergammMsg)>, StdError> = PENDINGACKS
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    Ok(PendingAcksResponse { pending: pending? })
}

pub fn do_ibc_packet_ack(
    deps: DepsMut,
    _env: Env,
    sequence: u64,
    error: Option<String>,
    response: Option<intergamm_bindings::msg::AckResponse>,
) -> Result<Response<IntergammMsg>, ContractError> {
    // save the message as acked
    ACKS.save(
        deps.storage,
        sequence,
        &intergamm_bindings::msg::AckValue {
            error: error.clone(),
            response: response.clone(),
        },
    )?;
    // remove the ack from pending
    PENDINGACKS.remove(deps.storage, sequence);
    Ok(Response::new()
        .add_attribute("error", error.unwrap_or("none".into()))
        .add_attribute("response", format!("{:?}", response)))
}

#[cfg(test)]
mod tests {}
