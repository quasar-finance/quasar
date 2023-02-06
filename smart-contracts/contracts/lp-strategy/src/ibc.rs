use crate::bond::{batch_bond, create_share};
use crate::error::{ContractError, Never, Trap};
use crate::helpers::{create_ibc_ack_submsg, get_ica_address, IbcMsgKind, IcaMessages};
use crate::ibc_lock::Lock;
use crate::ibc_util::{do_ibc_join_pool_swap_extern_amount_in, do_ibc_lock_tokens};
use crate::icq::calc_total_balance;
use crate::start_unbond::{batch_start_unbond, handle_start_unbond_ack};
use crate::state::{
    PendingBond, CHANNELS, CONFIG, IBC_LOCK, ICA_CHANNEL, LP_SHARES, OSMO_LOCK, PENDING_ACK,
    START_UNBOND_QUEUE, TRAPS, UNBONDING_CLAIMS,
};
use crate::unbond::{batch_unbond, finish_unbond, transfer_batch_unbond, PendingReturningUnbonds};
use cosmos_sdk_proto::cosmos::bank::v1beta1::QueryBalanceResponse;
use cosmos_sdk_proto::ibc::applications::transfer::v2::FungibleTokenPacketData;
use osmosis_std::types::osmosis::gamm::v1beta1::{
    MsgExitSwapShareAmountInResponse, MsgJoinSwapExternAmountInResponse,
    QueryCalcExitPoolCoinsFromSharesResponse,
};
use osmosis_std::types::osmosis::gamm::v2::QuerySpotPriceResponse;
use osmosis_std::types::osmosis::lockup::MsgLockTokensResponse;
use prost::Message;
use quasar_types::callback::{BondResponse, Callback};
use quasar_types::error::Error as QError;
use quasar_types::ibc::{
    enforce_order_and_version, ChannelInfo, ChannelType, HandshakeState, IcsAck,
};
use quasar_types::ica::handshake::enforce_ica_order_and_metadata;
use quasar_types::ica::packet::{ica_send, AckBody};
use quasar_types::ica::traits::Unpack;
use quasar_types::icq::{CosmosResponse, InterchainQueryPacketAck, ICQ_ORDERING};
use quasar_types::{ibc, ica::handshake::IcaMetadata, icq::ICQ_VERSION};

use cosmwasm_std::{
    entry_point, from_binary, to_binary, Attribute, Binary, Coin, DepsMut, Env, IbcBasicResponse,
    IbcChannel, IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg, IbcMsg, IbcPacket,
    IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse, IbcTimeout,
    StdError, Storage, Uint128, WasmMsg,
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
    let mut pending = PENDING_ACK.load(deps.storage, pkt.original_packet.sequence)?;
    match pending {
        // an transfer ack means we have sent funds to
        IbcMsgKind::Transfer(data) => {
            match handle_transfer_ack(deps.storage, env, ack_bin, &pkt, data.clone()) {
                Ok(response) => Ok(response),
                Err(err) => {
                    TRAPS.save(
                        deps.storage,
                        pkt.original_packet.sequence,
                        &Trap {
                            error: err.to_string(),
                            step: IbcMsgKind::Transfer(data),
                        },
                    )?;
                    Ok(IbcBasicResponse::new().add_attribute("trapped-error", err.to_string()))
                }
            }
        }
        IbcMsgKind::Ica(ref mut ica_kind) => {
            match handle_ica_ack(deps.storage, env, ack_bin, &pkt, ica_kind) {
                Ok(response) => Ok(response),
                Err(err) => {
                    TRAPS.save(
                        deps.storage,
                        pkt.original_packet.sequence,
                        &Trap {
                            error: err.to_string(),
                            step: IbcMsgKind::Ica(ica_kind.clone()),
                        },
                    )?;
                    Ok(IbcBasicResponse::new().add_attribute("trapped-error", err.to_string()))
                }
            }
        }
        IbcMsgKind::Icq => match handle_icq_ack(deps.storage, env, ack_bin, &pkt) {
            Ok(response) => Ok(response),
            Err(err) => {
                TRAPS.save(
                    deps.storage,
                    pkt.original_packet.sequence,
                    &Trap {
                        error: err.to_string(),
                        step: IbcMsgKind::Icq,
                    },
                )?;
                Ok(IbcBasicResponse::new().add_attribute("trapped-error", err.to_string()))
            }
        },
    }
}

pub fn handle_transfer_ack(
    storage: &mut dyn Storage,
    env: Env,
    _ack_bin: Binary,
    pkt: &IbcPacketAckMsg,
    pending: PendingBond,
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
        // TODO update share_out_min_amount to get a better estimate
        Uint128::one(),
        pending.bonds,
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
) -> Result<IbcBasicResponse, ContractError> {
    let ack: InterchainQueryPacketAck = from_binary(&ack_bin)?;

    let resp: CosmosResponse = CosmosResponse::decode(ack.data.0.as_ref())?;
    // we have only dispatched on query and a single kind at this point
    let balance = QueryBalanceResponse::decode(resp.responses[0].value.as_ref())?
        .balance
        .ok_or(ContractError::BaseDenomNotFound)?
        .amount;
    let quote_balance = QueryBalanceResponse::decode(resp.responses[0].value.as_ref())?
        .balance
        .ok_or(ContractError::BaseDenomNotFound)?
        .amount;
    let lp_balance = QueryBalanceResponse::decode(resp.responses[0].value.as_ref())?
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

    let total_lp = LP_SHARES.load(storage)?;

    let bond = batch_bond(storage, &env, total_balance)?;

    let start_unbond = batch_start_unbond(storage, &env, total_lp)?;

    let unbond = batch_unbond(storage, &env, total_balance, total_lp)?;

    let mut msges = Vec::new();
    let mut attrs = Vec::new();
    // if queues had items, msges should be some, so we add the ibc submessage, if there were no items in a queue, we don't have a submsg to add
    // if we have a bond, start_unbond or unbond msg, we lock the repsective lock
    if bond.is_some() {
        msges.push(bond.unwrap());
        attrs.push(Attribute::new("bond-status", "bonding"));
        IBC_LOCK.update(storage, |lock| -> Result<Lock, ContractError> {
            Ok(lock.lock_bond())
        })?;
    } else {
        attrs.push(Attribute::new("bond-status", "empty"));
    }

    if start_unbond.is_some() {
        msges.push(start_unbond.unwrap());
        attrs.push(Attribute::new("start-unbond-status", "starting-unbond"));
        IBC_LOCK.update(storage, |lock| -> Result<Lock, ContractError> {
            Ok(lock.lock_start_unbond())
        })?;
    } else {
        attrs.push(Attribute::new("start-unbond-status", "empty"));
    }

    if unbond.is_some() {
        msges.push(unbond.unwrap());
        attrs.push(Attribute::new("unbond-status", "unbonding"));
        IBC_LOCK.update(storage, |lock| -> Result<Lock, ContractError> {
            Ok(lock.lock_unbond())
        })?;
    } else {
        attrs.push(Attribute::new("unbond-status", "empty"));
    }

    Ok(IbcBasicResponse::new()
        .add_submessages(msges)
        .add_attributes(attrs))
}

pub fn handle_ica_ack(
    storage: &mut dyn Storage,
    env: Env,
    ack_bin: Binary,
    pkt: &IbcPacketAckMsg,
    ica_kind: &mut IcaMessages,
) -> Result<IbcBasicResponse, ContractError> {
    match ica_kind {
        IcaMessages::JoinSwapExternAmountIn(data) => {
            // TODO move the below locking logic to a separate function
            // get the ica address of the channel id
            let ica_addr = get_ica_address(storage, pkt.original_packet.src.channel_id.clone())?;
            let ack = AckBody::from_bytes(ack_bin.0.as_ref())?.to_any()?;
            let resp = MsgJoinSwapExternAmountInResponse::unpack(ack)?;
            let shares_out = Uint128::new(resp.share_out_amount.parse::<u128>()?);
            LP_SHARES.update(storage, |old| -> Result<Uint128, ContractError> {
                Ok(old.checked_add(shares_out)?)
            })?;

            let denom = CONFIG.load(storage)?.pool_denom;

            // TODO update queue raw amounts here
            data.update_raw_amount_to_lp(Uint128::new(resp.share_out_amount.parse::<u128>()?))?;

            let msg = do_ibc_lock_tokens(
                storage,
                ica_addr,
                vec![Coin {
                    denom,
                    amount: shares_out,
                }],
            )?;

            let outgoing = ica_send(
                msg,
                pkt.original_packet.src.channel_id.clone(),
                IbcTimeout::with_timestamp(env.block.time.plus_seconds(300)),
            )?;

            let msg = create_ibc_ack_submsg(
                storage,
                &IbcMsgKind::Ica(IcaMessages::LockTokens(data.clone())),
                outgoing,
            )?;
            Ok(IbcBasicResponse::new().add_submessage(msg))
        }
        // todo move the lock_tokens ack handling to a seperate function
        IcaMessages::LockTokens(data) => {
            let ack = AckBody::from_bytes(ack_bin.0.as_ref())?.to_any()?;
            let resp = MsgLockTokensResponse::unpack(ack)?;

            // save the lock id in the contract
            OSMO_LOCK.save(storage, &resp.id)?;

            let mut callbacks: Vec<WasmMsg> = vec![];
            for claim in &data.bonds {
                let share_amount = create_share(storage, claim.owner.clone(), claim.claim_amount)?;
                callbacks.push(WasmMsg::Execute {
                    contract_addr: claim.owner.to_string(),
                    msg: to_binary(&Callback::BondResponse(BondResponse {
                        share_amount,
                        bond_id: claim.bond_id.clone(),
                    }))?,
                    funds: vec![],
                })
            }

            // set the bond lock state to unlocked
            IBC_LOCK.update(storage, |old| -> Result<Lock, StdError> {
                Ok(old.unlock_bond())
            })?;

            // TODO, do we want to also check queue state? and see if we can already start a new execution?
            Ok(IbcBasicResponse::new()
                .add_messages(callbacks)
                .add_attribute("locked_tokens", ack_bin.to_base64())
                .add_attribute("lock_id", resp.id.to_string()))
        }
        IcaMessages::BeginUnlocking(data) => handle_start_unbond_ack(storage, &env, data),
        // TODO hook up the unbond ICA messages
        IcaMessages::ExitPool(data) => handle_exit_pool_ack(storage, &env, data, ack_bin),
        // TODO decide where we unlock the transfer ack unlock, here or in the ibc hooks receive
        IcaMessages::ReturnTransfer(data) => handle_return_transfer_ack(storage, data),
    }
}

fn handle_exit_pool_ack(
    storage: &mut dyn Storage,
    env: &Env,
    data: &mut PendingReturningUnbonds,
    ack_bin: Binary,
) -> Result<IbcBasicResponse, ContractError> {
    let msg: MsgExitSwapShareAmountInResponse =
        MsgExitSwapShareAmountInResponse::decode(ack_bin.0.as_ref())?;
    let total_tokens = Uint128::new(msg.token_out_amount.parse::<u128>()?);
    data.lp_to_local_denom(total_tokens)?;
    let sub_msg = transfer_batch_unbond(storage, env, data, total_tokens)?;
    Ok(IbcBasicResponse::new().add_submessage(sub_msg))
}

fn handle_return_transfer_ack(
    storage: &dyn Storage,
    data: &mut PendingReturningUnbonds,
) -> Result<IbcBasicResponse, ContractError> {
    let mut msgs: Vec<WasmMsg> = Vec::new();
    for pending in data.unbonds.iter() {
        let msg = finish_unbond(storage, pending)?;
        msgs.push(msg);
    }
    Ok(IbcBasicResponse::new()
        .add_attribute("callback-msgs", msgs.len().to_string())
        .add_messages(msgs)
        .add_attribute("return-transfer", "success"))
}

pub fn handle_failing_ack(
    _deps: DepsMut,
    _env: Env,
    _pkt: IbcPacketAckMsg,
    error: String,
) -> Result<IbcBasicResponse, ContractError> {
    // TODO we can expand error handling here to fetch the packet by the ack and add easy retries or something
    Ok(IbcBasicResponse::new().add_attribute("ibc-error", error.as_str()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_timeout(
    deps: DepsMut,
    _env: Env,
    msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse, ContractError> {
    // TODO: trap error like in acks
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
mod test {
    use super::*;
}
