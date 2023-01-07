use quasar_types::ibc::ChannelInfo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use cosmwasm_std::{CosmosMsg, IbcEndpoint, IbcPacket, Uint128, Addr};
use cw_storage_plus::{Item, Map};

use crate::{helpers::{IbcMsgKind, MsgKind}, error::{ContractError, Trap}};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    pub lock_period: Uint128,
    pub pool_id: u64,
    // pool_denom is the denom of the gamm pool on osmosis; eg gamm/pool/1
    pub pool_denom: String,
    // the denom here has to be the denom of the asset on quasar,
    // after proper hops, it should be the correct denom on osmosis
    pub denom: String,
}

pub(crate) const CONFIG: Item<Config> = Item::new("tmp");

pub(crate) const REPLIES: Map<u64, PendingAck> = Map::new("replies");
// Currently we only support one ICA channel to a single destination
pub(crate) const ICA_CHANNEL: Item<String> = Item::new("ica_channel");
pub(crate) const CHANNELS: Map<String, ChannelInfo> = Map::new("channels");
pub(crate) const PENDING_ACK: Map<u64, PendingAck> = Map::new("pending_acks");
// The map to store trapped errors,
pub(crate) const TRAPS: Map<String, Trap> = Map::new("traps");
pub(crate) const CLAIMS: Map<String, Claim> = Map::new("stubs");

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PendingAck {
    // the ibc msg of the sequence number
    pub kind: IbcMsgKind,
    // the address of the original caller
    pub address: Addr,
    // the amount of funds of the original call
    pub amount: Uint128,
}

impl PendingAck {
    pub fn update_kind(mut self, kind: IbcMsgKind) -> Self{
        self.kind = kind;
        self
    }
}

pub struct Claim {
    amount: Uint128
}

impl Claim {
    fn add(mut self, amount: Uint128) -> Result<Self, ContractError> {
        self.amount = self.amount.checked_add(amount)?;
        Ok(self)
    }
}
