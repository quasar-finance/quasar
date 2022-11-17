use crate::contract::do_ibc_lock_tokens;
use crate::error::{ContractError, Never};
use crate::helpers::{create_reply, create_submsg, MsgKind, IbcMsgKind, IcaMessages};
use crate::state::{CHANNELS, PENDING_ACK};
use osmosis_std::types::osmosis::gamm::v1beta1::MsgJoinSwapExternAmountInResponse;
use quasar_types::error::Error as QError;
use quasar_types::ibc::{
    enforce_order_and_version, ChannelInfo, ChannelType, HandshakeState, IcsAck,
};
use quasar_types::ica::enforce_ica_order_and_metadata;
use quasar_types::icq::ICQ_ORDERING;
use quasar_types::{ibc, ica::IcaMetadata, icq::ICQ_VERSION};

use cosmwasm_std::{
    attr, entry_point, from_binary, to_binary, BankMsg, Binary, CosmosMsg, DepsMut, Env,
    IbcAcknowledgement, IbcBasicResponse, IbcChannel, IbcChannelCloseMsg, IbcChannelConnectMsg,
    IbcChannelOpenMsg, IbcEndpoint, IbcOrder, IbcPacket, IbcPacketAckMsg, IbcPacketReceiveMsg,
    IbcPacketTimeoutMsg, IbcReceiveResponse, StdError, StdResult, Uint128, WasmMsg, Response, SubMsg,
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
        crate::helpers::IbcMsgKind::Ica(ica_kind) => {
            match ica_kind {
                crate::helpers::IcaMessages::JoinSwapExternAmountIn => {
                    let response: MsgJoinSwapExternAmountInResponse = from_binary(&ack_bin)?;
                    let msg = do_ibc_lock_tokens(deps.storage, response.share_out_amount)?;
                    let msg_kind = MsgKind::Ibc(IbcMsgKind::Ica(IcaMessages::LockTokens));
                    Ok(IbcBasicResponse::new().add_submessage(create_submsg(deps.storage, msg_kind, msg)?))
                },
                crate::helpers::IcaMessages::LockTokens => todo!(),
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
    todo!()
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

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::test_helpers::*;

//     use crate::contract::query_channel;
//     use cosmwasm_std::testing::mock_env;
//     use cosmwasm_std::{coins, to_vec, IbcEndpoint};

//     #[test]
//     fn check_ack_json() {
//         let success = StrategyAck::Result(b"1".into());
//         let fail = StrategyAck::Error("bad coin".into());

//         let success_json = String::from_utf8(to_vec(&success).unwrap()).unwrap();
//         assert_eq!(r#"{"result":"MQ=="}"#, success_json.as_str());

//         let fail_json = String::from_utf8(to_vec(&fail).unwrap()).unwrap();
//         assert_eq!(r#"{"error":"bad coin"}"#, fail_json.as_str());
//     }

//     #[test]
//     fn check_packet_json() {
//         let packet = StrategyPacket::new(
//             Uint128(12345),
//             "ucosm",
//             "cosmos1zedxv25ah8fksmg2lzrndrpkvsjqgk4zt5ff7n",
//             "wasm1fucynrfkrt684pm8jrt8la5h2csvs5cnldcgqc",
//         );
//         // Example message generated from the SDK
//         let expected = r#"{"amount":"12345","denom":"ucosm","receiver":"wasm1fucynrfkrt684pm8jrt8la5h2csvs5cnldcgqc","sender":"cosmos1zedxv25ah8fksmg2lzrndrpkvsjqgk4zt5ff7n"}"#;

//         let encdoded = String::from_utf8(to_vec(&packet).unwrap()).unwrap();
//         assert_eq!(expected, encdoded.as_str());
//     }
// }
