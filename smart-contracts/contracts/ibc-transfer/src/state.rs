use cosmwasm_std::{from_binary, to_vec, Binary, StdResult, Storage};
use cw_storage_plus::{Item, Map};
use cosmwasm_std::{CosmosMsg, IbcPacket, Uint128};

pub(crate) const REPLIES: Map<u64, IbcPacket> = Map::new("replies");
pub(crate) const PENDING_ACK: Map<u64, IbcPacket> = Map::new("pending_acks");
