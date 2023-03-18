use std::collections::HashMap;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, IbcPacketAckMsg, StdResult, Uint128};

use quasar_types::ibc::ChannelInfo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    error::Trap,
    helpers::{IbcMsgKind, SubMsgKind},
    ibc_lock,
    state::{Config, LpCache, Unbond},
};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {
    pub lock_period: u64,
    pub pool_id: u64,
    pub pool_denom: String,
    pub local_denom: String,
    pub base_denom: String,
    pub quote_denom: String,
    // TODO should this be outgoing_transfer_channel?
    pub transfer_channel: String,
    // TODO rename to return_transfer_channel
    pub return_source_channel: String,
    pub expected_connection: String,
}

impl InstantiateMsg {
    pub fn validate(&self) -> StdResult<()> {
        Ok(())
    }
}

#[cw_serde]
pub struct MigrateMsg {
    pub config: Config,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Channels {},
    Config {},
    IcaAddress {},
    Lock {},
    LpShares {},
    PrimitiveShares {},
    IcaBalance {},
    IcaChannel {},
    TrappedErrors {},
    UnbondingClaim { addr: Addr, id: String },
    ListUnbondingClaims {},
    ListBondingClaims {},
    ListPrimitiveShares {},
    ListPendingAcks {},
    ListReplies {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ListBondingClaimsResponse {
    pub bonds: HashMap<Addr, (String, Uint128)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ListRepliesResponse {
    pub replies: HashMap<u64, SubMsgKind>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ListPrimitiveSharesResponse {
    pub shares: HashMap<Addr, Uint128>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ListPendingAcksResponse {
    pub pending: HashMap<u64, IbcMsgKind>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ListUnbondingClaimsResponse {
    pub unbonds: HashMap<(Addr, String), Unbond>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UnbondingClaimResponse {
    pub unbond: Unbond,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ChannelsResponse {
    pub channels: Vec<ChannelInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TrappedErrorsResponse {
    pub errors: Vec<(u64, Trap)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LpSharesResponse {
    pub lp_shares: LpCache,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ConfigResponse {
    pub config: Config,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct LockResponse {
    pub lock: ibc_lock::Lock,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct IcaAddressResponse {
    pub address: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct PrimitiveSharesResponse {
    pub total: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct IcaBalanceResponse {
    pub amount: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct IcaChannelResponse {
    pub channel: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Bond { id: String },
    StartUnbond { id: String, share_amount: Uint128 },
    Unbond { id: String },
    // accept a dispatched transfer from osmosis
    AcceptReturningFunds { id: u64 },
    // try to close a channel where a timout occured
    CloseChannel { channel_id: String },
    ReturnTransfer { amount: Uint128 },
    Ack { ack: IbcPacketAckMsg },
    TryIcq {},
}
