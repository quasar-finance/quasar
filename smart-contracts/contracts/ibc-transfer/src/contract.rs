use cosmwasm_std::{
    coin, entry_point, Binary, CosmosMsg, Deps, DepsMut, Env, IbcMsg, IbcTimeout, IbcTimeoutBlock,
    MessageInfo, Reply, Response, StdError, StdResult, SubMsg, Storage,
};

use crate::{helpers::{parse_seq,  MsgKind, create_reply, IbcMsgKind}, error::ContractError};

use protobuf::Message as ProtoMessage;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{PENDING_ACK, REPLIES};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    deps.api.debug("WASMDEBUG: Instantiate");
    Ok(Response::default())
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Transfer { to_address: String, channel: String },
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    deps.api
        .debug(format!("WASMDEBUG: execute: received msg: {:?}", msg).as_str());
    match msg {
        ExecuteMsg::Transfer {
            to_address,
            channel,
        } => execute_transfer(deps, env, info, channel, to_address),
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Type1 {
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Type2 {
    pub data: String,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    // Save the ibc message together with the sequence number, to be handled properly later at the ack
    let kind = REPLIES.load(deps.storage, msg.id)?;
    match kind {
        MsgKind::Ibc(ibc_kind) => {
            let seq = parse_seq(msg)?;
            PENDING_ACK.save(deps.storage, seq, &ibc_kind)?;
        }
    }
    Ok(Response::default())
}

pub fn do_ibc_lock_tokens(
    deps: &mut dyn Storage,
    token_amount: String,
) -> Result<CosmosMsg, ContractError> {
    todo!()
}

fn execute_transfer(
    deps:DepsMut,
    env: Env,
    info: MessageInfo,
    channel: String,
    to_address: String,
) -> StdResult<Response> {
    if info.funds.len() != 1 {
        return Err(StdError::GenericErr {
            msg: "invalid number of denoms sent (send one)".to_string(),
        });
    }

    let funds = info.funds[0].clone();
    let transfer = IbcMsg::Transfer {
        channel_id: channel.clone(),
        to_address: to_address.clone(),
        amount: funds,
        timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(300)),
    };
    Ok(create_reply(deps.storage, MsgKind::Ibc(IbcMsgKind::Transfer), transfer)?
        .add_attribute("ibc-tranfer-channel", channel)
        .add_attribute("ibc-transfer-receiver", to_address))
}
