use crate::{
    proto::CosmosResponse,
    state::{Origin, PENDING_QUERIES, QUERY_RESULT_COUNTER, REPLIES},
    ContractError,
};
use cosmwasm_std::{
    attr, DepsMut, Env, IbcBasicResponse, IbcPacket, Order, Reply, Response, StdError, StdResult,
};

pub(crate) fn handle_reply_sample(deps: DepsMut, msg: Reply) -> StdResult<Response> {
    let val = msg
        .result
        .into_result()
        .map_err(|msg| StdError::GenericErr { msg })?;

    let event = val
        .events
        .iter()
        .find(|e| e.ty == "send_packet")
        .ok_or(StdError::NotFound {
            kind: "send_packet_event".into(),
        })?;

    // here we can do further stuff with a succesful package if necessary, in this case we can simply
    // save the package, under the sequence number and channel id
    let seq = event
        .attributes
        .iter()
        .find(|attr| attr.key == "packet_sequence")
        .ok_or(StdError::NotFound {
            kind: "packet_sequence".into(),
        })?;
    let s = seq.value.parse::<u64>().map_err(|e| StdError::ParseErr {
        target_type: "u64".into(),
        msg: e.to_string(),
    })?;
    let channel = event
        .attributes
        .iter()
        .find(|attr| attr.key == "packet_src_channel")
        .ok_or(StdError::NotFound {
            kind: "packet_src_channel".into(),
        })?;

    PENDING_QUERIES.save(deps.storage, (s, &channel.value), &Origin::Sample)?;
    Ok(Response::new().add_attribute("reply_registered", msg.id.to_string()))
}

pub fn set_reply(deps: DepsMut, origin: &Origin) -> Result<u64, ContractError> {
    let last = REPLIES
        .range(deps.storage, None, None, Order::Descending)
        .next();
    let mut id: u64 = 0;
    if let Some(val) = last {
        id = val?.0;
    }

    // register the message in the replies for handling
    REPLIES.save(deps.storage, id, origin)?;
    // send response
    Ok(id)
}

// for our sample origin callback, we increment the query counter and leave it at that
pub fn handle_sample_callback(
    deps: DepsMut,
    _env: Env,
    response: CosmosResponse,
    _original: IbcPacket,
) -> Result<IbcBasicResponse, ContractError> {
    let attrs = vec![
        attr("action", "acknowledge"),
        attr("num_messages", response.responses.len().to_string()),
        attr("success", "true"),
    ];

    // Store result counter.
    let mut counter = QUERY_RESULT_COUNTER.load(deps.storage)?;
    counter += response.responses.len() as u64;
    QUERY_RESULT_COUNTER.save(deps.storage, &counter)?;
    Ok(IbcBasicResponse::new().add_attributes(attrs))
}

#[cfg(test)]
mod test {}
