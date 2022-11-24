use prost::bytes::Bytes;
use prost::Message;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    attr, entry_point, from_binary, Binary, DepsMut, Env, IbcBasicResponse, IbcChannel,
    IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg, IbcOrder, IbcPacket,
    IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse,
};

use crate::error::{ContractError, Never};
use crate::helpers::handle_sample_callback;
use crate::proto::CosmosResponse;
use crate::state::{ChannelInfo, Origin, CHANNEL_INFO, PENDING_QUERIES};

pub const ICQ_VERSION: &str = "icq-1";
pub const ICQ_ORDERING: IbcOrder = IbcOrder::Unordered;

/// This is compatible with the JSON serialization
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
pub struct InterchainQueryPacketAck {
    pub data: Binary,
}

impl InterchainQueryPacketAck {
    pub fn new(data: Binary) -> Self {
        InterchainQueryPacketAck { data }
    }

    pub fn validate(&self) -> Result<(), ContractError> {
        Ok(())
    }
}

/// This is a generic ICS acknowledgement format.
/// Proto defined here: https://github.com/cosmos/cosmos-sdk/blob/v0.42.0/proto/ibc/core/channel/v1/channel.proto#L141-L147
/// This is compatible with the JSON serialization
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum IcsAck {
    Result(Binary),
    Error(String),
}

#[cfg_attr(not(feature = "library"), entry_point)]
/// enforces ordering and versioning constraints
pub fn ibc_channel_open(
    _deps: DepsMut,
    _env: Env,
    msg: IbcChannelOpenMsg,
) -> Result<(), ContractError> {
    enforce_order_and_version(msg.channel(), msg.counterparty_version())?;
    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
/// record the channel in CHANNEL_INFO
pub fn ibc_channel_connect(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelConnectMsg,
) -> Result<IbcBasicResponse, ContractError> {
    // we need to check the counter party version in try and ack (sometimes here)
    enforce_order_and_version(msg.channel(), msg.counterparty_version())?;

    let channel: IbcChannel = msg.into();
    let info = ChannelInfo {
        id: channel.endpoint.channel_id,
        counterparty_endpoint: channel.counterparty_endpoint,
        connection_id: channel.connection_id,
    };
    CHANNEL_INFO.save(deps.storage, &info.id, &info)?;

    Ok(IbcBasicResponse::default())
}

fn enforce_order_and_version(
    channel: &IbcChannel,
    counterparty_version: Option<&str>,
) -> Result<(), ContractError> {
    if channel.version != ICQ_VERSION {
        return Err(ContractError::InvalidIbcVersion {
            version: channel.version.clone(),
        });
    }
    if let Some(version) = counterparty_version {
        if version != ICQ_VERSION {
            return Err(ContractError::InvalidIbcVersion {
                version: version.to_string(),
            });
        }
    }
    if channel.order != ICQ_ORDERING {
        return Err(ContractError::OnlyOrderedChannel {});
    }
    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_close(
    _deps: DepsMut,
    _env: Env,
    _channel: IbcChannelCloseMsg,
) -> Result<IbcBasicResponse, ContractError> {
    // TODO: what to do here?
    // we will have locked funds that need to be returned somehow
    unimplemented!();
}

#[cfg_attr(not(feature = "library"), entry_point)]
/// Check to see if we have any balance here
/// We should not return an error if possible, but rather an acknowledgement of failure
pub fn ibc_packet_receive(
    _deps: DepsMut,
    _env: Env,
    _msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, Never> {
    // Contract does not handle packets/queries.
    unimplemented!();
}

#[cfg_attr(not(feature = "library"), entry_point)]
/// check if success or failure and update balance, or return funds
pub fn ibc_packet_ack(
    deps: DepsMut,
    env: Env,
    msg: IbcPacketAckMsg,
) -> Result<IbcBasicResponse, ContractError> {
    // Design decision: should we trap error like in receive?
    // TODO: unsure... as it is now a failed ack handling would revert the tx and would be
    // retried again and again. is that good?
    let ics_msg: IcsAck = from_binary(&msg.acknowledgement.data)?;
    match ics_msg {
        IcsAck::Result(data) => on_packet_success(deps, data, msg.original_packet, env),
        IcsAck::Error(err) => on_packet_failure(deps, msg.original_packet, err),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
/// return fund to original sender (same as failure in ibc_packet_ack)
pub fn ibc_packet_timeout(
    deps: DepsMut,
    _env: Env,
    msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse, ContractError> {
    // TODO: trap error like in receive? (same question as ack above)
    let packet = msg.packet;
    on_packet_failure(deps, packet, "timeout".to_string())
}

fn on_packet_success(
    deps: DepsMut,
    data: Binary,
    original: IbcPacket,
    env: Env,
) -> Result<IbcBasicResponse, ContractError> {
    let ack: InterchainQueryPacketAck = from_binary(&data)?;

    let buf = Bytes::copy_from_slice(ack.data.as_slice());
    let resp: CosmosResponse = match CosmosResponse::decode(buf) {
        Ok(resp) => resp,
        Err(_) => return Err(ContractError::DecodingFail {}),
    };

    // load the msg from the pending queries so we know what to do
    let origin =
        PENDING_QUERIES.load(deps.storage, (original.sequence, &original.src.channel_id))?;
    match origin {
        Origin::Sample => Ok(handle_sample_callback(deps, env, resp, original)?),
    }
}

fn on_packet_failure(
    _deps: DepsMut,
    _packet: IbcPacket,
    err: String,
) -> Result<IbcBasicResponse, ContractError> {
    let attributes = vec![
        attr("action", "acknowledge"),
        attr("success", "false"),
        attr("error", err),
    ];

    Ok(IbcBasicResponse::new().add_attributes(attributes))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::state::QUERY_RESULT_COUNTER;
    use crate::ContractError;

    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use cosmwasm_std::{Binary, IbcAcknowledgement, IbcEndpoint, IbcPacket, IbcTimeout, Timestamp};

    #[test]
    fn test_ibc_packet_ack() -> Result<(), ContractError> {
        let mut deps = mock_dependencies();
        let timeout = IbcTimeout::with_timestamp(Timestamp::from_nanos(0));
        let src = IbcEndpoint {
            port_id: "port-0".to_string(),
            channel_id: "channel-0".to_string(),
        };
        let dest = IbcEndpoint {
            port_id: "port-1".to_string(),
            channel_id: "channel-1".to_string(),
        };
        let packet = IbcPacket::new(Binary::default(), src, dest, 0, timeout);

        // save the sequence number for the callback
        PENDING_QUERIES.save(deps.as_mut().storage, (0, "channel-0"), &Origin::Sample)?;
        let ack = IbcAcknowledgement::new(Binary::from_base64(
            "eyJyZXN1bHQiOiJleUprWVhSaElqb2lRMmRaU1VWcmFXUnpaMFU5SW4wPSJ9",
        )?);
        let msg = IbcPacketAckMsg::new(ack, packet);
        QUERY_RESULT_COUNTER.save(deps.as_mut().storage, &0)?; // Save a 0
        match ibc_packet_ack(deps.as_mut(), mock_env(), msg) {
            Ok(_) => {
                assert_eq!(QUERY_RESULT_COUNTER.load(deps.as_mut().storage)?, 1);
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}
