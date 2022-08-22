#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cosmwasm_std::{
    IbcBasicResponse, IbcChannelCloseMsg, IbcChannelOpenMsg, IbcPacketAckMsg, IbcPacketReceiveMsg,
    IbcPacketTimeoutMsg, IbcReceiveResponse,
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
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<IntergammMsg>, ContractError> {
    match msg {
        ExecuteMsg::SendToken {} => execute_send_token(),
        ExecuteMsg::JoinPool {} => {
            todo!()
        }
        ExecuteMsg::AckTriggered {} => do_ibc_packet_ack(deps, _env),
    }
}

pub fn execute_send_token() -> Result<Response<IntergammMsg>, ContractError> {
    Ok(Response::new().add_message(IntergammMsg::SendToken {
        creator: "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec".to_string(),
        destination_local_zone_id: "test1".to_string(),
        sender: "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec".to_string(),
        receiver: "quasar1sqlsc5024sszglyh7pswk5hfpc5xtl77gqjwec".to_string(),
        coin: Coin::new(1, "denom"),
    }))
}

pub fn execute_deposit(info: MessageInfo) ->Result<Response<IntergammMsg>, ContractError> {
    let funds  = cw_utils::must_pay(&info, "uqsar")?;
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

#[cfg_attr(not(feature = "library"), entry_point)]
/// check if success or failure and update balance, or return funds
pub fn ibc_packet_ack(
    deps: DepsMut,
    _env: Env,
    msg: IbcPacketAckMsg,
) -> Result<IbcBasicResponse, ContractError> {
    let triggered = ACKTRIGGERED.load(deps.storage)?;
    ACKTRIGGERED.save(deps.storage, &(triggered + 1))?;
    Ok(IbcBasicResponse::new().add_attribute("ack", "succes"))
}

pub fn do_ibc_packet_ack(deps: DepsMut, _env: Env) -> Result<Response<IntergammMsg>, ContractError> {
    let triggered = ACKTRIGGERED.load(deps.storage)?;
    ACKTRIGGERED.save(deps.storage, &(triggered + 1))?;
    Ok(Response::new().add_attribute("ack tiggered", (triggered+1).to_string()))
}

#[entry_point]
/// enforces ordering and versioning constraints
pub fn ibc_channel_open(deps: DepsMut, env: Env, msg: IbcChannelOpenMsg) -> StdResult<()> {
    todo!()
}

#[entry_point]
pub fn ibc_channel_close(
    deps: DepsMut,
    env: Env,
    msg: IbcChannelCloseMsg,
) -> StdResult<IbcBasicResponse> {
    todo!()
}

#[entry_point]
pub fn ibc_packet_receive(
    deps: DepsMut,
    env: Env,
    msg: IbcPacketReceiveMsg,
) -> StdResult<IbcReceiveResponse> {
    todo!()
}

#[entry_point]
/// never should be called as we do not send packets
pub fn ibc_packet_timeout(
    deps: DepsMut,
    env: Env,
    msg: IbcPacketTimeoutMsg,
) -> StdResult<IbcBasicResponse> {
    todo!()
}

#[cfg(test)]
mod tests {}
