use cosmwasm_std::{entry_point, Addr, StdError, Uint128};
use cosmwasm_std::{DepsMut, Env, Reply, Response, StdResult};

use crate::error::ContractError;
use crate::helpers::{parse_seq, IbcMsgKind};
use crate::state::{PENDING_ACK, REPLIES, WITHDRAWABLE};

// if the msg is not an error, the underlying contract succeeded, we then deduct the contracts deposit from withdrawable
pub fn handle_execute_reply(deps: DepsMut, msg: Reply, owner: Addr, amount: Uint128) -> StdResult<()> {
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

pub fn handle_ibc_reply(deps: DepsMut, msg: Reply, kind: IbcMsgKind) -> StdResult<()> {
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
        msg: format!("{:?}", err.to_string()),
    })?;

    Ok(PENDING_ACK.save(deps.storage, seq, &kind)?)
}
