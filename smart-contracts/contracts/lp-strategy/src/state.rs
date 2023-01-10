use quasar_types::ibc::ChannelInfo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

use crate::{
    error::{ContractError, Trap},
    helpers::IbcMsgKind,
};

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
    // the denom on the Quasar chain0
    pub local_denom: String,
}

pub(crate) const CONFIG: Item<Config> = Item::new("tmp");

// IBC related state items
//
pub(crate) const DEPOSIT_SEQ: Item<Uint128> = Item::new("deposit_seq");
pub(crate) const REPLIES: Map<u64, PendingAck> = Map::new("replies");
// Currently we only support one ICA channel to a single destination
pub(crate) const ICA_CHANNEL: Item<String> = Item::new("ica_channel");
// We also support one ICQ channel to Osmosis at the moment
pub(crate) const ICQ_CHANNEL: Item<String> = Item::new("icq_channel");

// The channel over which to transfer the tokens,
pub(crate) const TRANSFER_CHANNEL: Item<String> = Item::new("transfer_channel");

pub(crate) const CHANNELS: Map<String, ChannelInfo> = Map::new("channels");
pub(crate) const PENDING_ACK: Map<u64, PendingAck> = Map::new("pending_acks");
// The map to store trapped errors,
pub(crate) const TRAPS: Map<String, Trap> = Map::new("traps");

// all vault related state items

// transferred funds is the amount of funds we have already transferred. The first Uint128 is the nth transfer, the second is the amount of that transfer
pub(crate) const TRANSFERRED_FUNDS: Map<u128, Uint128> = Map::new("transferred_funds");

// The total amount of we have used to join the pool, but are not yet locked
pub(crate) const JOINED_FUNDS: Map<u128, Uint128> = Map::new("locked_funds");

// The total amount of funds we have locked in the pool, since this contract only accepts one denom, we can use an Uint128
pub(crate) const LOCKED_FUNDS: Map<u128, Uint128> = Map::new("locked_funds");
pub(crate) const CLAIMS: Map<Addr, Uint128> = Map::new("claims");
pub(crate) const SHARES: Map<Addr, Uint128> = Map::new("shares");

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PendingAck {
    // the ibc msg of the sequence number
    pub kind: IbcMsgKind,
    // the address of the original caller
    pub address: Addr,
    // the amount of funds in contract denom of the original call
    pub amount: Uint128,
    // the id of the sequence of this ack
    pub id: Uint128,
}

impl PendingAck {
    pub fn update_kind(mut self, kind: IbcMsgKind) -> Self {
        self.kind = kind;
        self
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Claim {
    amount: Uint128,
}