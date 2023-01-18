use crate::error::{ContractError, Never, Trap};
use crate::helpers::{create_ibc_ack_submsg, get_ica_address, IbcMsgKind, IcaMessages};
use crate::lock::Lock;
use crate::state::{PendingAck, CHANNELS, CONFIG, ICA_CHANNEL, LOCK, PENDING_ACK, TRAPS};
use crate::strategy::{do_ibc_join_pool_swap_extern_amount_in, do_ibc_lock_tokens};
use crate::vault::{calc_total_balance, create_share, handle_query_ack};
use cosmos_sdk_proto::cosmos::bank::v1beta1::QueryBalanceResponse;
use cosmos_sdk_proto::ibc::applications::transfer::v2::FungibleTokenPacketData;
use osmosis_std::types::osmosis::gamm::v1beta1::{
    MsgJoinSwapExternAmountInResponse, QueryCalcExitPoolCoinsFromSharesResponse,
};
use osmosis_std::types::osmosis::gamm::v2::QuerySpotPriceResponse;
use osmosis_std::types::osmosis::lockup::MsgLockTokensResponse;
use prost::Message;
use quasar_types::error::Error as QError;
use quasar_types::ibc::{
    enforce_order_and_version, ChannelInfo, ChannelType, HandshakeState, IcsAck,
};
use quasar_types::ica::handshake::enforce_ica_order_and_metadata;
use quasar_types::ica::packet::AckBody;
use quasar_types::ica::traits::Unpack;
use quasar_types::icq::{CosmosResponse, InterchainQueryPacketAck, ICQ_ORDERING};
use quasar_types::{ibc, ica::handshake::IcaMetadata, icq::ICQ_VERSION};

use cosmwasm_std::{
    entry_point, from_binary, to_binary, Binary, Coin, DepsMut, Env, IbcBasicResponse, IbcChannel,
    IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg, IbcMsg, IbcPacket,
    IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse, IbcTimeout,
    StdError, Storage, Uint128,
};

/// enforces ordering and versioning constraints, this combines ChanOpenInit and ChanOpenTry
#[cfg_attr(not(feature = "library"), entry_point)]
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

/// record the channel in CHANNEL_INFO, this combines the ChanOpenAck and ChanOpenConfirm steps
#[cfg_attr(not(feature = "library"), entry_point)]
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

            // once we have an Open ICA channel, save it under ICA channel, if a channel already exists, reject incoming OPENS
            let channel = ICA_CHANNEL.may_load(deps.storage)?;
            if channel.is_some() {
                return Err(ContractError::IcaChannelAlreadySet);
            }

            ICA_CHANNEL.save(deps.storage, &msg.channel().endpoint.channel_id)?;

            info.channel_type = ChannelType::Ica {
                channel_ty,
                counter_party_address: addr,
            }
        }
        ChannelType::Ics20 { channel_ty: _ } => unimplemented!(),
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

/// The lp-strategy cannot receive any packets
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_receive(
    _deps: DepsMut,
    _env: Env,
    _msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, Never> {
    // Contract does not handle packets/queries.
    unimplemented!();
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_ack(
    deps: DepsMut,
    env: Env,
    msg: IbcPacketAckMsg,
) -> Result<IbcBasicResponse, ContractError> {
    // TODO: trap error like in receive?
    // pro's acks happen anyway, cons?
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
    let pending = PENDING_ACK.load(deps.storage, pkt.original_packet.sequence)?;
    match &pending.kind {
        // an transfer ack means we have sent funds to
        crate::helpers::IbcMsgKind::Transfer => {
            match handle_transfer_ack(deps.storage, env, ack_bin, &pkt, pending.clone()) {
                Ok(response) => Ok(response),
                Err(err) => {
                    TRAPS.save(
                        deps.storage,
                        pkt.original_packet.sequence,
                        &Trap {
                            error: err.to_string(),
                            step: IbcMsgKind::Transfer,
                            deposits: pending.deposits,
                        },
                    )?;
                    Ok(IbcBasicResponse::new().add_attribute("trapped-error", err.to_string()))
                }
            }
        }
        crate::helpers::IbcMsgKind::Ica(_) => {
            match handle_ica_ack(deps.storage, env, ack_bin, &pkt, pending.clone()) {
                Ok(response) => Ok(response),
                Err(err) => {
                    TRAPS.save(
                        deps.storage,
                        pkt.original_packet.sequence,
                        &Trap {
                            error: err.to_string(),
                            step: IbcMsgKind::Transfer,
                            deposits: pending.deposits,
                        },
                    )?;
                    Ok(IbcBasicResponse::new().add_attribute("trapped-error", err.to_string()))
                }
            }
        }
        crate::helpers::IbcMsgKind::Icq => {
            match handle_icq_ack(deps.storage, env, ack_bin, &pkt, pending.clone()) {
                Ok(response) => Ok(response),
                Err(err) => {
                    TRAPS.save(
                        deps.storage,
                        pkt.original_packet.sequence,
                        &Trap {
                            error: err.to_string(),
                            step: IbcMsgKind::Transfer,
                            deposits: pending.deposits,
                        },
                    )?;
                    Ok(IbcBasicResponse::new().add_attribute("trapped-error", err.to_string()))
                }
            }
        }
    }
}

pub fn handle_transfer_ack(
    storage: &mut dyn Storage,
    env: Env,
    _ack_bin: Binary,
    pkt: &IbcPacketAckMsg,
    pending: PendingAck,
) -> Result<IbcBasicResponse, ContractError> {
    // once the ibc transfer to the ICA account has succeeded, we send the join pool message
    // we need to save and fetch
    let config = CONFIG.load(storage)?;
    let ica_channel = ICA_CHANNEL.load(storage)?;
    let original: FungibleTokenPacketData = Message::decode(pkt.original_packet.data.as_ref())?;
    let amount = Uint128::new(original.amount.as_str().parse::<u128>()?);

    let msg = do_ibc_join_pool_swap_extern_amount_in(
        storage,
        env,
        ica_channel,
        config.pool_id,
        original.denom.clone(),
        amount,
        Uint128::one(),
        pending.deposits,
    )?;

    Ok(IbcBasicResponse::new().add_submessage(msg).add_attribute(
        "transfer-ack",
        format!("{}-{}", original.amount, original.denom),
    ))
}

pub fn handle_icq_ack(
    storage: &mut dyn Storage,
    env: Env,
    ack_bin: Binary,
    _pkt: &IbcPacketAckMsg,
    pending: PendingAck,
) -> Result<IbcBasicResponse, ContractError> {
    let ack: InterchainQueryPacketAck = from_binary(&ack_bin)?;

    let resp: CosmosResponse = CosmosResponse::decode(ack.data.0.as_ref())?;
    // we have only dispatched on query and a single kind at this point
    let balance = QueryBalanceResponse::decode(resp.responses[0].value.as_ref())?
        .balance
        .ok_or(ContractError::BaseDenomNotFound)?
        .amount;
    let exit_pool =
        QueryCalcExitPoolCoinsFromSharesResponse::decode(resp.responses[1].value.as_ref())?;
    let spot_price = QuerySpotPriceResponse::decode(resp.responses[2].value.as_ref())?.spot_price;

    let total_balance = calc_total_balance(
        storage,
        Uint128::new(balance.parse()?),
        exit_pool.tokens_out,
        Uint128::new(spot_price.parse()?),
    )?;
    let transfer = handle_query_ack(storage, env, pending.clone(), total_balance)?;
    Ok(IbcBasicResponse::new()
        .add_submessage(transfer)
        .add_attribute("transfer-icq-total", total_balance))
}

pub fn handle_ica_ack(
    storage: &mut dyn Storage,
    env: Env,
    ack_bin: Binary,
    pkt: &IbcPacketAckMsg,
    pending: PendingAck,
) -> Result<IbcBasicResponse, ContractError> {
    match &pending.kind {
        IbcMsgKind::Ica(ica_kind) => {
            match ica_kind {
                IcaMessages::JoinSwapExternAmountIn => {
                    // TODO move the below locking logic to a separate function
                    // get the ica address of the channel id
                    let ica_addr =
                        get_ica_address(storage, pkt.original_packet.src.channel_id.clone())?;
                    let ack = AckBody::from_bytes(ack_bin.0.as_ref())?.to_any()?;
                    let resp = MsgJoinSwapExternAmountInResponse::unpack(ack)?;
                    let denom = CONFIG.load(storage)?.base_denom;

                    let ica_pkt = do_ibc_lock_tokens(
                        storage,
                        ica_addr,
                        vec![Coin {
                            denom,
                            amount: Uint128::new(resp.share_out_amount.parse::<u128>()?),
                        }],
                    )?;
                    let ibc_pkt = IbcMsg::SendPacket {
                        channel_id: pkt.original_packet.src.channel_id.clone(),
                        data: to_binary(&ica_pkt)?,
                        timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(300)),
                    };

                    let msg = create_ibc_ack_submsg(
                        storage,
                        pending.update_kind(IbcMsgKind::Ica(IcaMessages::LockTokens)),
                        ibc_pkt,
                    )?;
                    Ok(IbcBasicResponse::new().add_submessage(msg))
                }
                IcaMessages::LockTokens => {
                    let ack = AckBody::from_bytes(ack_bin.0.as_ref())?.to_any()?;
                    let resp = MsgLockTokensResponse::unpack(ack)?;

                    for claim in pending.deposits {
                        create_share(storage, claim.owner, claim.claim_amount)?;
                    }

                    // set the lock state to unlocked
                    LOCK.save(storage, &Lock::Unlocked)?;

                    // TODO, do we want to also check queue state? and see if we can already start a new execution?
                    Ok(IbcBasicResponse::new()
                        .add_attribute("locked_tokens", ack_bin.to_base64())
                        .add_attribute("lock_id", resp.id.to_string()))
                }
            }
        }
        _ => unimplemented!(),
    }
}

pub fn handle_failing_ack(
    _deps: DepsMut,
    _env: Env,
    _pkt: IbcPacketAckMsg,
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
    _deps: DepsMut,
    _packet: IbcPacket,
    _error: String,
) -> Result<IbcBasicResponse, ContractError> {
    todo!()
}

#[cfg(test)]
mod test {}
