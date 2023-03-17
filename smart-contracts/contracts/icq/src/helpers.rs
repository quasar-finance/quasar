use crate::{
    msg::InterchainQueryPacketData,
    proto::{CosmosQuery, CosmosResponse},
    state::{CHANNEL_INFO, CONFIG, QUERY_RESULT_COUNTER},
    ContractError,
};
use cosmos_sdk_proto::tendermint::abci::RequestQuery;
use cosmwasm_std::{
    attr, DepsMut, Env, IbcBasicResponse, IbcPacket, Order, Reply, Response, StdError, StdResult,
    Storage, Timestamp,
};
use prost::Message;

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

    Ok(Response::new().add_attribute("reply_registered", msg.id.to_string()))
}

pub fn prepare_query(
    storage: &dyn Storage,
    env: Env,
    channel: &str,
) -> Result<Timestamp, ContractError> {
    // ensure the requested channel is registered
    if !CHANNEL_INFO.has(storage, channel) {
        return Err(ContractError::NoSuchChannel { id: channel.into() });
    }
    let config = CONFIG.load(storage)?;
    // delta from user is in seconds
    let timeout_delta = config.default_timeout;

    // timeout is in nanoseconds
    Ok(env.block.time.plus_seconds(timeout_delta))
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

#[derive(Clone, Debug, PartialEq)]
pub struct Query {
    requests: Vec<RequestQuery>,
}

impl Query {
    pub fn new() -> Query {
        Query {
            requests: Vec::new(),
        }
    }

    pub fn add_request(mut self, data: Vec<u8>, path: String) -> Self {
        self.requests.push(RequestQuery {
            data,
            path,
            height: 0,
            prove: false,
        });
        self
    }

    pub fn encode(self) -> Vec<u8> {
        CosmosQuery {
            requests: self.requests,
        }
        .encode_to_vec()
    }

    pub fn encode_pkt(self) -> InterchainQueryPacketData {
        InterchainQueryPacketData {
            data: self.encode(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn single_query_works() {
        let req = RequestQuery {
            data: vec![1, 0, 1, 0],
            path: "/cosmos.bank.v1beta1.Query/AllBalances".into(),
            height: 0,
            prove: false,
        };

        let data = Query::new().add_request(req.data.clone(), req.path.clone());

        assert_eq!(
            data,
            Query {
                requests: vec![req.clone()]
            }
        );
        assert_eq!(
            data.encode(),
            CosmosQuery {
                requests: vec![req]
            }
            .encode_to_vec()
        )
    }

    #[test]
    pub fn multiple_query_works() {
        let req1 = RequestQuery {
            data: vec![1, 0, 1, 0],
            path: "/cosmos.bank.v1beta1.Query/AllBalances".into(),
            height: 0,
            prove: false,
        };
        let req2 = RequestQuery {
            data: vec![1, 0, 0, 0],
            path: "/cosmos.bank.v1beta1.Query/Balance".into(),
            height: 0,
            prove: false,
        };

        let data = Query::new()
            .add_request(req1.data.clone(), req1.path.clone())
            .add_request(req2.data.clone(), req2.path.clone());

        assert_eq!(
            data,
            Query {
                requests: vec![req1.clone(), req2.clone()]
            }
        );
        assert_eq!(
            data.encode(),
            CosmosQuery {
                requests: vec![req1, req2]
            }
            .encode_to_vec()
        )
    }
}
