use crate::{
    error::ContractError,
    ibc_lock::Lock,
    state::{PendingBond, PendingSingleUnbond, CHANNELS, IBC_LOCK, REPLIES, SHARES},
    unbond::PendingReturningUnbonds,
};
use cosmwasm_std::{Binary, IbcMsg, Order, StdError, Storage, SubMsg, Uint128};
use prost::Message;
use quasar_types::ibc::MsgTransferResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub fn get_total_shares(storage: &dyn Storage) -> Result<Uint128, ContractError> {
    // workaround for a div-by-zero error on multi-asset vault side
    let mut sum = Uint128::one();
    for val in SHARES.range(storage, None, None, Order::Ascending) {
        sum = sum.checked_add(val?.1)?;
    }
    Ok(sum)
}

pub fn get_ica_address(store: &dyn Storage, channel_id: String) -> Result<String, ContractError> {
    let chan = CHANNELS.load(store, channel_id)?;
    match chan.channel_type {
        quasar_types::ibc::ChannelType::Icq { channel_ty: _ } => Err(ContractError::NoIcaChannel),
        quasar_types::ibc::ChannelType::Ica {
            channel_ty: _,
            counter_party_address,
        } => {
            if let Some(addr) = counter_party_address {
                Ok(addr)
            } else {
                Err(ContractError::NoCounterpartyIcaAddress)
            }
        }
        quasar_types::ibc::ChannelType::Ics20 { channel_ty: _ } => Err(ContractError::NoIcaChannel),
    }
}

pub fn check_icq_channel(storage: &dyn Storage, channel: String) -> Result<(), ContractError> {
    let chan = CHANNELS.load(storage, channel)?;
    match chan.channel_type {
        quasar_types::ibc::ChannelType::Icq { channel_ty: _ } => Ok(()),
        quasar_types::ibc::ChannelType::Ica {
            channel_ty: _,
            counter_party_address: _,
        } => Err(ContractError::NoIcqChannel),
        quasar_types::ibc::ChannelType::Ics20 { channel_ty: _ } => Err(ContractError::NoIcqChannel),
    }
}

pub fn create_ibc_ack_submsg(
    storage: &mut dyn Storage,
    pending: &IbcMsgKind,
    msg: IbcMsg,
) -> Result<SubMsg, StdError> {
    let last = REPLIES.range(storage, None, None, Order::Descending).next();
    let mut id: u64 = 0;
    if let Some(val) = last {
        id = val?.0;
    }
    // register the message in the replies for handling
    REPLIES.save(storage, id, pending)?;
    Ok(SubMsg::reply_always(msg, id))
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum IbcMsgKind {
    Transfer {
        pending: PendingBond,
        amount: Uint128,
    },
    Ica(IcaMessages),
    Icq,
}

// All enums supported by this contract
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum IcaMessages {
    JoinSwapExternAmountIn(PendingBond),
    LockTokens(PendingBond),
    BeginUnlocking(Vec<PendingSingleUnbond>),
    ExitPool(PendingReturningUnbonds),
    ReturnTransfer(PendingReturningUnbonds),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MsgKind {
    Ibc(IbcMsgKind),
}

pub(crate) fn parse_seq(data: Binary) -> Result<u64, ContractError> {
    let resp = MsgTransferResponse::decode(data.0.as_slice())?;
    Ok(resp.seq)
}

pub(crate) fn unlock_on_error(
    storage: &mut dyn Storage,
    kind: IbcMsgKind,
) -> Result<(), ContractError> {
    match kind {
        IbcMsgKind::Transfer {
            pending: _,
            amount: _,
        } => {
            IBC_LOCK.update(storage, |lock| {
                Ok::<Lock, ContractError>(lock.unlock_bond())
            })?;
            Ok(())
        }
        IbcMsgKind::Ica(ica) => match ica {
            IcaMessages::JoinSwapExternAmountIn(_) => {
                IBC_LOCK.update(storage, |lock| {
                    Ok::<Lock, ContractError>(lock.unlock_bond())
                })?;
                Ok(())
            }
            IcaMessages::LockTokens(_) => {
                IBC_LOCK.update(storage, |lock| {
                    Ok::<Lock, ContractError>(lock.unlock_bond())
                })?;
                Ok(())
            }
            IcaMessages::BeginUnlocking(_) => {
                IBC_LOCK.update(storage, |lock| {
                    Ok::<Lock, ContractError>(lock.unlock_start_unbond())
                })?;
                Ok(())
            }
            IcaMessages::ExitPool(_) => {
                IBC_LOCK.update(storage, |lock| {
                    Ok::<Lock, ContractError>(lock.unlock_unbond())
                })?;
                Ok(())
            }
            IcaMessages::ReturnTransfer(_) => {
                IBC_LOCK.update(storage, |lock| {
                    Ok::<Lock, ContractError>(lock.unlock_unbond())
                })?;
                Ok(())
            }
        },
        IbcMsgKind::Icq => {
            IBC_LOCK.update(storage, |lock| {
                Ok::<Lock, ContractError>(lock.unlock_bond().unlock_start_unbond().unlock_unbond())
            })?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_reply_seq() {
        let seq = 35;
        let resp = Binary::from(MsgTransferResponse { seq }.encode_to_vec());
        let parsed_seq = parse_seq(resp).unwrap();
        assert_eq!(seq, parsed_seq)
    }
}
