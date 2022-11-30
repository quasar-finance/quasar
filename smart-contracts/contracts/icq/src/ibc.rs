use prost::Message;
use quasar_types::ibc::{enforce_order_and_version, IcsAck};
use quasar_types::icq::{CosmosResponse, InterchainQueryPacketAck, ICQ_ORDERING, ICQ_VERSION};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    attr, entry_point, from_binary, Binary, DepsMut, Env, IbcBasicResponse, IbcChannel,
    IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg, IbcOrder, IbcPacket,
    IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse,
};

use crate::error::{ContractError, Never};
use crate::helpers::handle_sample_callback;
use crate::state::{ChannelInfo, Origin, CHANNEL_INFO, PENDING_QUERIES};

#[cfg_attr(not(feature = "library"), entry_point)]
/// enforces ordering and versioning constraints
pub fn ibc_channel_open(
    _deps: DepsMut,
    _env: Env,
    msg: IbcChannelOpenMsg,
) -> Result<(), ContractError> {
    enforce_order_and_version(
        msg.channel(),
        msg.counterparty_version(),
        ICQ_VERSION,
        ICQ_ORDERING,
    )?;
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
    enforce_order_and_version(
        msg.channel(),
        msg.counterparty_version(),
        ICQ_VERSION,
        ICQ_ORDERING,
    )?;

    let channel: IbcChannel = msg.into();
    let info = ChannelInfo {
        id: channel.endpoint.channel_id,
        counterparty_endpoint: channel.counterparty_endpoint,
        connection_id: channel.connection_id,
    };
    CHANNEL_INFO.save(deps.storage, &info.id, &info)?;

    Ok(IbcBasicResponse::default())
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

    let resp: CosmosResponse = match CosmosResponse::decode(ack.data.as_slice()) {
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
