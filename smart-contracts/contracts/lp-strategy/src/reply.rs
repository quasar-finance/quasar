use cosmwasm_std::{entry_point, Addr, StdError, Uint128};
use cosmwasm_std::{DepsMut, Env, Reply, Response, StdResult};

use crate::error::ContractError;
use crate::helpers::{parse_seq, IbcMsgKind};
use crate::state::{PENDING_ACK, REPLIES, WITHDRAWABLE};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> StdResult<Response> {
    // Save the ibc message together with the sequence number, to be handled properly later at the ack, we can pass the ibc_kind one to one
    // TODO this needs and error check and error handling
    let kind = REPLIES.load(deps.storage, reply.id)?;
    match kind {
        crate::helpers::MsgKind::Ibc(ibc_kind) => handle_ibc_reply(deps, reply, ibc_kind)?,
        crate::helpers::MsgKind::WasmExecute(owner, amount) => {
            handle_execute_reply(deps, reply, owner, amount)?
        }
    }
    Ok(Response::default())
}

// if the msg is not an error, the underlying contract succeeded, we then deduct the contracts deposit from withdrawable
fn handle_execute_reply(deps: DepsMut, msg: Reply, owner: Addr, amount: Uint128) -> StdResult<()> {
    // if the result is an error
    if msg.result.is_ok() {
        let withdraw = WITHDRAWABLE.load(deps.storage, owner.clone())?;
        if withdraw == amount {
            WITHDRAWABLE.remove(deps.storage, owner);
        } else {
            WITHDRAWABLE.save(deps.storage, owner, &withdraw.checked_sub(amount)?)?;
        }
    }
    Ok(())
}

fn handle_ibc_reply(deps: DepsMut, msg: Reply, kind: IbcMsgKind) -> StdResult<()> {
    let data = msg
        .result
        .into_result()
        .map_err(|msg| StdError::GenericErr { msg })?
        .data
        .ok_or(ContractError::NoReplyData)
        .map_err(|err| StdError::GenericErr {
            msg: err.to_string(),
        })?;

    let seq = parse_seq(data).map_err(|err| StdError::GenericErr {
        msg: err.to_string(),
    })?;

    Ok(PENDING_ACK.save(deps.storage, seq, &kind)?)
}
