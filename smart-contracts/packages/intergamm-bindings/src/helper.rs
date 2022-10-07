use cosmwasm_std::{
    Attribute, Order, Reply, Response, StdError, StdResult, Storage, SubMsg, SubMsgResponse, Env, DepsMut, Deps, Addr,
};

use crate::{msg::{IntergammMsg, AckResponse, AckValue}, state::{REPLIES, CALLBACKADDRESS, PENDINGACKS, ACKS}, error::ContractError, };

pub fn set_callback_addr(deps: DepsMut, callback_addr: &str) -> Result<(), ContractError>{
    Ok(CALLBACKADDRESS.save(deps.storage, &deps.api.addr_validate(callback_addr)?)?)
}

pub fn check_callback_addr(deps: Deps, sender: Addr) -> Result<(), ContractError> {
    let callback = CALLBACKADDRESS.load(deps.storage)?;
    if sender != callback {
        return Err(ContractError::Unauthorized { sender: sender.to_string(), expected: callback.to_string() });
    }
    Ok(())
}

pub fn create_intergamm_msg(
    storage: &mut dyn Storage,
    msg: IntergammMsg,
) -> Result<Response<IntergammMsg>, StdError> {
    let last = REPLIES.range(storage, None, None, Order::Descending).next();
    let mut id: u64 = 0;
    if let Some(val) = last {
        id = val?.0;
    }
    // register the message in the replies for handling
    REPLIES.save(storage, id, &msg)?;
    Ok(Response::new().add_submessage(SubMsg::reply_always(msg, id)))
}

pub fn ack(deps: DepsMut, sequence: u64, error: &Option<String>, response: &Option<AckResponse>) -> Result<(), ContractError> {
    ACKS.save(
        deps.storage,
        sequence,
        &AckValue {
            error: error.clone(),
            response: response.clone(),
        },
    )?;
    PENDINGACKS.remove(deps.storage, sequence);
    Ok(())
}

// handle_reply provides a basic handle function for responses to intergamm messages
// the acks map is the map where
pub fn handle_reply(
    store: &mut dyn Storage,
    env: Env,
    msg: Reply,
) -> StdResult<Response> {
    let res = msg.result.into_result();
    if let Ok(ok) = res {
        // do something with the ok msg
        let original = REPLIES.load(store, msg.id)?;
        match original {
            IntergammMsg::SendToken {
                destination_local_zone_id,
                receiver: _,
                coin: _,
            } => Ok(Response::new().add_attribute("send_token", "sender").add_attribute("destination", destination_local_zone_id)),
            IntergammMsg::TestScenario { scenario } => Ok( Response::new().add_attribute("testing scenario", scenario)),
            IntergammMsg::RegisterIcaOnZone {
                zone_id,
            } => Ok(Response::new()
                .add_attribute("register_interchain_account", env.contract.address)
                .add_attribute("zone_id", zone_id)),
            IntergammMsg::JoinSwapExternAmountIn {
                ref connection_id,
                timeout_timestamp: _,
                pool_id: _,
                share_out_min_amount: _,
                token_in: _,
            } => {
                store_pending_ack(ok, connection_id, store, &original)
            }
            IntergammMsg::JoinPool { ref connection_id, timeout_timestamp: _, pool_id: _,share_out_amount: _, token_in_maxs: _ } => store_pending_ack(ok, connection_id,  store, &original),
            IntergammMsg::LockTokens { ref connection_id, timeout_timestamp: _, duration: _, coins: _ } => store_pending_ack(ok, connection_id,  store, &original),
            IntergammMsg::ExitSwapExternAmountOut { ref connection_id, timeout_timestamp: _, pool_id: _, share_in_amount: _, token_out_mins: _ } => store_pending_ack(ok, connection_id,  store, &original),
            IntergammMsg::BeginUnlocking { ref connection_id, timeout_timestamp: _, id: _, coins: _ } => store_pending_ack(ok, connection_id,  store, &original),
            IntergammMsg::ExitPool { ref connection_id, timeout_timestamp: _, pool_id: _, share_in_amount: _, token_out_mins: _ } => store_pending_ack(ok, connection_id,  store, &original),
        }
    } else {
        Err(StdError::GenericErr {
            msg: format!("reply status: {}", res.unwrap_err()),
        })
    }
}

fn store_pending_ack(msg: SubMsgResponse, connection_id: &str, store: &mut dyn Storage, original: &IntergammMsg) -> Result<Response, StdError> {
    // to get the sequence number, we look for the event type send_packet under the key packet_sequence and register the sequence number
    let e = msg
        .events
        .iter()
        .find(|e| e.ty == "send_packet")
        .ok_or_else(|| StdError::GenericErr {
            msg: "packet event not found".into(),
        })?;

    // we do some sanity checks here to see if the attributes of the packet correspond with the intergamm msg
    if connection_id != find_attr(&e.attributes, "packet_connection")?.value {
        return Err(StdError::GenericErr {
            msg: "connection_id is not equal to packet connection".into(),
        });
    }
    let seq = find_attr(&e.attributes, "packet_sequence")?;

    let s = seq.value.parse::<u64>().map_err(|e| StdError::ParseErr {
        target_type: "u64".into(),
        msg: e.to_string(),
    })?;
    
    PENDINGACKS.save(store, s, original)?;
    Ok(Response::new().add_attribute("added pending ack", s.to_string()))
}

fn find_attr<'a>(attributes: &'a [Attribute], key: &str) -> Result<&'a Attribute, StdError> {
    attributes
        .iter()
        .find(|attr| attr.key == key)
        .ok_or_else(|| StdError::GenericErr {
            msg: format!("packet does not containt attribute {}", key),
        })
}
