use crate::state::REPLIES;
use cosmwasm_std::{CosmosMsg, Order, Reply, Response, StdError, Storage, SubMsg};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub fn create_reply(
    storage: &mut dyn Storage,
    msg_kind: MsgKind,
    msg: impl Into<CosmosMsg>,
) -> Result<Response, StdError> {
    let last = REPLIES.range(storage, None, None, Order::Descending).next();
    let mut id: u64 = 0;
    if let Some(val) = last {
        id = val?.0;
    }
    // register the message in the replies for handling
    REPLIES.save(storage, id, &msg_kind)?;
    Ok(Response::new().add_submessage(SubMsg::reply_always(msg, id)))
}

pub fn create_submsg(
    storage: &mut dyn Storage,
    msg_kind: MsgKind,
    msg: impl Into<CosmosMsg>,
) -> Result<SubMsg, StdError> {
    let last = REPLIES.range(storage, None, None, Order::Descending).next();
    let mut id: u64 = 0;
    if let Some(val) = last {
        id = val?.0;
    }
    // register the message in the replies for handling
    REPLIES.save(storage, id, &msg_kind)?;
    Ok(SubMsg::reply_always(msg, id))
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum IbcMsgKind {
    Transfer,
    Ica(IcaMessages),
    Icq,
}

// All enums supported by this contract
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum IcaMessages {
    JoinSwapExternAmountIn,
    LockTokens,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MsgKind {
    Ibc(IbcMsgKind),
}

pub(crate) fn parse_seq(reply: Reply) -> Result<u64, StdError> {
    reply
        .result
        .into_result()
        .map_err(|msg| StdError::GenericErr { msg })?
        .events
        .iter()
        .find(|e| e.ty == "send_packet")
        .ok_or(StdError::NotFound {
            kind: "send_packet_event".into(),
        })?
        .attributes
        .iter()
        .find(|attr| attr.key == "packet_sequence")
        .ok_or(StdError::NotFound {
            kind: "packet_sequence".into(),
        })?
        .value
        .parse::<u64>()
        .map_err(|e| StdError::ParseErr {
            target_type: "u64".into(),
            msg: e.to_string(),
        })
}

#[cfg(test)]
mod tests {
    // TODO write some tests for helpers
}
