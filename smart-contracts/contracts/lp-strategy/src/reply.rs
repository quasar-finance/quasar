use cosmwasm_std::StdError;
use cosmwasm_std::{DepsMut, Reply, Response};
use quasar_types::callback::Callback;

use crate::error::{ContractError, Trap};
use crate::helpers::{parse_seq, unlock_on_error, IbcMsgKind};
use crate::state::{PENDING_ACK, REPLIES, TRAPS};

pub fn handle_ibc_reply(
    deps: DepsMut,
    msg: Reply,
    pending: IbcMsgKind,
) -> Result<Response, ContractError> {
    let data = msg
        .result
        .into_result()
        .map_err(|msg| StdError::GenericErr {
            msg: format!("submsg error: {msg:?}"),
        })?
        .data
        .ok_or(ContractError::NoReplyData)
        .map_err(|_| StdError::NotFound {
            kind: "reply-data".to_string(),
        })?;

    let seq = parse_seq(data).map_err(|err| StdError::SerializeErr {
        source_type: "protobuf-decode".to_string(),
        msg: err.to_string(),
    })?;

    PENDING_ACK.save(deps.storage, seq, &pending)?;

    // cleanup the REPLIES state item
    REPLIES.remove(deps.storage, msg.id);

    Ok(Response::default()
        .add_attribute("pending-msg", seq.to_string())
        .add_attribute("step", format!("{pending:?}")))
}

pub fn handle_ack_reply(deps: DepsMut, msg: Reply, seq: u64) -> Result<Response, ContractError> {
    let mut resp = Response::new();

    // if we have an error in our Ack execution, the submsg saves the error in TRAPS and (should) rollback
    // the entire state of the ack execution,
    if let Err(error) = msg.result.into_result() {
        let step = PENDING_ACK.load(deps.storage, seq)?;
        unlock_on_error(deps.storage, &step)?;

        // reassignment needed since add_attribute
        resp = resp.add_attribute("trapped-error", error.as_str());

        TRAPS.save(deps.storage, seq, &Trap { error, step })?;
    }

    // cleanup the REPLIES state item
    REPLIES.remove(deps.storage, msg.id);
    Ok(resp)
}

pub fn handle_callback_reply(
    deps: DepsMut,
    msg: Reply,
    _callback: Callback,
) -> Result<Response, ContractError> {
    // TODO: if error, add manual withdraws to lp-strategy

    // cleanup the REPLIES state item
    REPLIES.remove(deps.storage, msg.id);
    Ok(Response::new())
}
