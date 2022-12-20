use crate::contract::do_ibc_lock_tokens;
use crate::error::{ContractError, Never};
use crate::helpers::{
    create_reply, create_submsg, get_ica_address, IbcMsgKind, IcaMessages, MsgKind,
};
use crate::state::{CHANNELS, PENDING_ACK};
use osmosis_std::types::cosmos::base::v1beta1::Coin;
use osmosis_std::types::osmosis::gamm::v1beta1::MsgJoinSwapExternAmountInResponse;
use osmosis_std::types::osmosis::lockup::{MsgLockTokens, MsgLockTokensResponse};
use quasar_types::error::Error as QError;
use quasar_types::ibc::{
    enforce_order_and_version, ChannelInfo, ChannelType, HandshakeState, IcsAck,
};
use quasar_types::ica::handshake::enforce_ica_order_and_metadata;
use quasar_types::ica::packet::AckBody;
use quasar_types::ica::traits::Unpack;
use quasar_types::icq::ICQ_ORDERING;
use quasar_types::{ibc, ica::handshake::IcaMetadata, icq::ICQ_VERSION};

use cosmwasm_std::{
    attr, entry_point, from_binary, to_binary, BankMsg, Binary, CosmosMsg, DepsMut, Env,
    IbcAcknowledgement, IbcBasicResponse, IbcChannel, IbcChannelCloseMsg, IbcChannelConnectMsg,
    IbcChannelOpenMsg, IbcEndpoint, IbcMsg, IbcOrder, IbcPacket, IbcPacketAckMsg,
    IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse, IbcTimeout, Response, StdError,
    StdResult, SubMsg, Uint128, WasmMsg,
};

#[cfg_attr(not(feature = "library"), entry_point)]
/// enforces ordering and versioning constraints, this combines ChanOpenInit and ChanOpenTry
pub fn ibc_channel_open(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelOpenMsg,
) -> Result<(), ContractError> {
    // save the channel as an channel in ChanOpenInit, we support inits from icq and ica channels
    if msg.channel().version == ICQ_VERSION {
        handle_icq_channel(deps, msg.channel().clone())?;
    } else {
        handle_ica_channel(deps, msg.channel().clone())?;
    }

    Ok(())
}

fn handle_icq_channel(deps: DepsMut, channel: IbcChannel) -> Result<(), ContractError> {
    ibc::enforce_order_and_version(&channel, None, &channel.version, channel.order.clone())?;
    // save the channel state here
    let info = ChannelInfo {
        id: channel.endpoint.channel_id.clone(),
        counterparty_endpoint: channel.counterparty_endpoint,
        connection_id: channel.connection_id,
        channel_type: ChannelType::Icq {
            channel_ty: channel.version,
        },
        handshake_state: HandshakeState::Init,
    };
    CHANNELS.save(deps.storage, channel.endpoint.channel_id, &info)?;
    Ok(())
}

fn handle_ica_channel(deps: DepsMut, channel: IbcChannel) -> Result<(), ContractError> {
    let metadata: IcaMetadata = serde_json_wasm::from_str(&channel.version).map_err(|error| {
        QError::InvalidIcaMetadata {
            raw_metadata: channel.version.clone(),
            error: error.to_string(),
        }
    })?;

    enforce_ica_order_and_metadata(&channel, None, &metadata)?;
    // save the current state of the initializing channel
    let info = ChannelInfo {
        id: channel.endpoint.channel_id.clone(),
        counterparty_endpoint: channel.counterparty_endpoint,
        connection_id: channel.connection_id,
        channel_type: ChannelType::Ica {
            channel_ty: metadata,
            counter_party_address: None,
        },
        handshake_state: HandshakeState::Init,
    };
    CHANNELS.save(deps.storage, channel.endpoint.channel_id, &info)?;
    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
/// record the channel in CHANNEL_INFO, this combines the ChanOpenAck and ChanOpenConfirm steps
pub fn ibc_channel_connect(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelConnectMsg,
) -> Result<IbcBasicResponse, ContractError> {
    // try to fetch the connecting channel, we should error if it does not exist\
    let mut info: ChannelInfo = CHANNELS
        .load(deps.storage, msg.channel().endpoint.channel_id.clone())
        .map_err(|err| StdError::GenericErr {
            msg: err.to_string(),
        })?;
    // we need to check the counter party version in try and ack (sometimes here)
    // TODO we can wrap this match in a function in our ibc package

    // TODO think of a better datastructure so we dont have to parse ICA channels like this
    match info.channel_type {
        ChannelType::Icq { ref channel_ty } => enforce_order_and_version(
            msg.channel(),
            msg.counterparty_version(),
            channel_ty.as_str(),
            ICQ_ORDERING,
        )?,
        ChannelType::Ica {
            channel_ty,
            counter_party_address: _,
        } => {
            let counter_party_metadata = enforce_ica_order_and_metadata(
                msg.channel(),
                msg.counterparty_version(),
                &channel_ty,
            )?;

            if counter_party_metadata.is_none() {
                return Err(ContractError::QError(QError::NoCounterpartyIcaAddress));
            }
            let counter_party = counter_party_metadata.unwrap();
            // at this point, we expect a counterparty address, if it's none, we have to error
            if counter_party.address().is_none() {
                return Err(ContractError::NoCounterpartyIcaAddress);
            }
            let addr = counter_party.address();
            if addr.is_none() {
                return Err(ContractError::NoCounterpartyIcaAddress);
            }
            info.channel_type = ChannelType::Ica {
                channel_ty,
                counter_party_address: addr,
            }
        }
        ChannelType::Ics20 { ref channel_ty } => todo!(),
    }

    info.handshake_state = HandshakeState::Open;

    CHANNELS.save(
        deps.storage,
        msg.channel().endpoint.channel_id.clone(),
        &info,
    )?;

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
    // TODO: trap error like in receive?
    let ack: IcsAck = from_binary(&msg.acknowledgement.data)?;
    match ack {
        IcsAck::Result(val) => handle_succesful_ack(deps, env, msg, val),
        IcsAck::Error(err) => handle_failing_ack(deps, env, msg, err),
    }
}

pub fn handle_succesful_ack(
    deps: DepsMut,
    env: Env,
    pkt: IbcPacketAckMsg,
    ack_bin: Binary,
) -> Result<IbcBasicResponse, ContractError> {
    let kind = PENDING_ACK.load(deps.storage, pkt.original_packet.sequence)?;
    match kind {
        crate::helpers::IbcMsgKind::Transfer => todo!(),
        crate::helpers::IbcMsgKind::Ica(ica_kind) => match ica_kind {
            crate::helpers::IcaMessages::JoinSwapExternAmountIn => {
                // TODO move the below locking logic to a separate function
                // get the ica address of the channel id
                let ica_addr =
                    get_ica_address(deps.storage, pkt.original_packet.src.channel_id.clone())?;
                deps.api.debug(ack_bin.to_base64().as_ref());
                let ack = AckBody::from_bytes(ack_bin.0.as_ref())?.to_any()?;
                let resp = MsgJoinSwapExternAmountInResponse::unpack(ack)?;

                let ica_pkt = do_ibc_lock_tokens(
                    deps.storage,
                    ica_addr,
                    vec![Coin {
                        denom: "gamm/pool/1".to_string(),
                        amount: resp.share_out_amount,
                    }],
                )?;
                let ibc_pkt = IbcMsg::SendPacket {
                    channel_id: pkt.original_packet.src.channel_id,
                    data: to_binary(&ica_pkt)?,
                    timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(300)),
                };

                let msg_kind = MsgKind::Ibc(IbcMsgKind::Ica(IcaMessages::LockTokens));
                let msg = create_submsg(deps.storage, msg_kind, ibc_pkt)?;
                Ok(IbcBasicResponse::new().add_submessage(msg))
            }
            crate::helpers::IcaMessages::LockTokens => {
                let ack = AckBody::from_bytes(ack_bin.0.as_ref())?.to_any()?;
                let resp = MsgLockTokensResponse::unpack(ack)?;

                Ok(IbcBasicResponse::new().add_attribute("locked_tokens", ack_bin.to_base64()).add_attribute("lock_id", resp.id.to_string()))
            }
        },
        crate::helpers::IbcMsgKind::Icq => todo!(),
    }
}

pub fn handle_failing_ack(
    deps: DepsMut,
    env: Env,
    pkt: IbcPacketAckMsg,
    error: String,
) -> Result<IbcBasicResponse, ContractError> {
    // TODO we can expand error handling here to fetch the packet by the
    Ok(IbcBasicResponse::new().add_attribute("ibc-error", error.as_str()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
/// return fund to original sender (same as failure in ibc_packet_ack)
pub fn ibc_packet_timeout(
    deps: DepsMut,
    _env: Env,
    msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse, ContractError> {
    // TODO: trap error like in receive?
    on_packet_failure(deps, msg.packet, "timeout".to_string())?;
    Ok(IbcBasicResponse::default())
}

fn on_packet_failure(
    deps: DepsMut,
    packet: IbcPacket,
    error: String,
) -> Result<IbcBasicResponse, ContractError> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    use cosmos_sdk_proto::ibc::core::channel::v1::Acknowledgement;
    use cosmos_sdk_proto::Any;
    use cosmwasm_std::testing::mock_env;
    use cosmwasm_std::{coins, to_vec, IbcEndpoint};
    use prost::Message;
}
