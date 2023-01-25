use crate::{
    error::ContractError,
    state::{PendingAck, CHANNELS, REPLIES},
};
use cosmwasm_std::{Binary, CosmosMsg, Order, StdError, Storage, SubMsg};
use prost::Message;
use quasar_types::ibc::MsgTransferResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    pending: &PendingAck,
    msg: impl Into<CosmosMsg>,
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
    Transfer,
    Ica(IcaMessages),
    Icq,
}

// All enums supported by this contract
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum IcaMessages {
    JoinSwapExternAmountIn,
    LockTokens,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MsgKind {
    Ibc(IbcMsgKind),
}

pub(crate) fn parse_seq(data: Binary) -> Result<u64, ContractError> {
    let resp = MsgTransferResponse::decode(data.0.as_slice())?;
    return Ok(resp.seq);
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
