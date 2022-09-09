use cosmwasm_std::{Order, Storage, StdError, Response, SubMsg, Reply, StdResult, Binary, Attribute};
use cw_storage_plus::Map;

use crate::{state::{REPLIES, STATUS, Status}, msg::IntergammMsg};

pub fn create_intergamm_msg(storage: &mut dyn Storage, msg: IntergammMsg) -> Result<Response<IntergammMsg>, StdError> {
    let last = REPLIES.range(storage, None, None, Order::Descending).next();
    let mut id = 0;
    if let Some(val) = last  {
        id = val?.0;
    }
    // register the message in the replies for handling
    REPLIES.save(storage, id, &msg)?;
    Ok(Response::new().add_submessage(SubMsg::reply_always(msg, id)))
}


// TODO(Laurens) expand handle reply
/// handle_reply provides a basic handle function for responses to intergamm messages
/// the acks map is the map where 
pub fn handle_reply(store: &mut dyn Storage, msg: Reply, acks: Map<u64, IntergammMsg>) -> StdResult<Response> {
    let mut res = msg.result.into_result();
    if let Ok(ok) = res {
        // do something with the ok msg
        let original = REPLIES.load(store, msg.id)?;
        match original {
            IntergammMsg::SendToken { creator, destination_local_zone_id, sender, receiver, coin } => todo!(),
            IntergammMsg::TestScenario { creator, scenario } => todo!(),
            // if RegisterInterchainAccount was succesful, we do not get a sequence number. The most we can do is register that we are opening the channel
            IntergammMsg::RegisterInterchainAccount { creator, connection_id } => {
                return handle_intergamm_status(store);
            },
            IntergammMsg::JoinSwapExternAmountIn { ref creator, ref connection_id, ref timeout_timestamp, pool_id, share_out_min_amount, ref token_in } => {
                // to get the sequence number, we look for the event type send_packet under the key packet_sequence and register the sequence number
                let e = ok.events.iter().find(|e| e.ty == "send_packet").ok_or_else(|| StdError::GenericErr { msg: "packet event not found".into() })?;
                // we do some sanity checks here to see if the attributes of the packet correspond with the intergamm msg
                if connection_id.clone() != find_attr(&e.attributes, "packet_connection")?.value {
                    return Err(StdError::GenericErr { msg: "connection_id is not equal to packet connection".into() });
                }

                let seq = find_attr(&e.attributes, "packet_sequence")?;
                // parse the seq value to an uin64
                let p = seq.value.parse::<u64>().map_err(|e| StdError::ParseErr { target_type: "u64".into(), msg: e.to_string() })?;
                // TODO once the closures are setup, add closures to for acks here
                acks.save(store, p, &original)?;
                Ok(Response::new().add_attribute("added pending ack", p.to_string()))
            },
        }
    } else {
        Err(StdError::GenericErr { msg: res.unwrap_err() })
    }
}

fn find_attr<'a>(attributes: &'a Vec<Attribute>, key: &str) -> Result<&'a Attribute, StdError> {
    attributes.iter().find(|attr| attr.key == key).ok_or_else(|| StdError::GenericErr { msg: format!("packet does not containt attribute {}", key)})
}

fn handle_intergamm_status(store: &mut dyn Storage) -> StdResult<Response> {
    let status = STATUS.load(store)?;
    match status {
        // if there is no interchain account ready for use, move the status to init
        Status::Unopened => {
            STATUS.save(store, &Status::Init)?;
            return Ok(Response::new().add_attribute("intergamm-status", "init"));
        },
        // if the interchain account currently is in the init state, we should check whether the connection is already open and change the status
        Status::Init => {
            // for now, we just move the status to open
            // since any other intergamm calls will error anyway if the channel is not open, this would be a semantically nice addition
            STATUS.save(store, &Status::Open)?;
            return Ok(Response::new().add_attribute("intergamm-status", "open"));
        },
        Status::Open => {
            // TODO (laurens) add queries to the intergamm bindings
            todo!()
        },
        // TODO decide what we do if a channel is closed, probably open it
        Status::Closed => todo!(),
    }
}