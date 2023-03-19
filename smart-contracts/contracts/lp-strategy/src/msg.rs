use std::collections::HashMap;

use cosmwasm_schema::{cw_serde, QueryResponses};
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

#[cw_serde]
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

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ChannelsResponse)]
    Channels {},
    #[returns(ConfigResponse)]
    Config {},
    #[returns(IcaAddressResponse)]
    IcaAddress {},
    #[returns(LockResponse)]
    Lock {},
    #[returns(LpSharesResponse)]
    LpShares {},
    #[returns(PrimitiveSharesResponse)]
    PrimitiveShares {},
    #[returns(IcaBalanceResponse)]
    IcaBalance {},
    #[returns(IcaChannelResponse)]
    IcaChannel {},
    #[returns(TrappedErrorsResponse)]
    TrappedErrors {},
    #[returns(UnbondingClaimResponse)]
    UnbondingClaim { addr: Addr, id: String },
    #[returns(ListUnbondingClaimsResponse)]
    ListUnbondingClaims {},
    #[returns(ListBondingClaimsResponse)]
    ListBondingClaims {},
    #[returns(ListPrimitiveSharesResponse)]
    ListPrimitiveShares {},
    #[returns(ListPendingAcksResponse)]
    ListPendingAcks {},
    #[returns(ListRepliesResponse)]
    ListReplies {},
}

#[cw_serde]
pub struct ListBondingClaimsResponse {
    pub bonds: HashMap<(Addr, String), Uint128>,
}

#[cw_serde]
pub struct ListRepliesResponse {
    pub replies: HashMap<u64, SubMsgKind>,
}

#[cw_serde]
pub struct ListPrimitiveSharesResponse {
    pub shares: HashMap<Addr, Uint128>,
}

#[cw_serde]
pub struct ListPendingAcksResponse {
    pub pending: HashMap<u64, IbcMsgKind>,
}

#[cw_serde]
pub struct ListUnbondingClaimsResponse {
    pub unbonds: HashMap<(Addr, String), Unbond>,
}

#[cw_serde]
pub struct UnbondingClaimResponse {
    pub unbond: Unbond,
}

#[cw_serde]
pub struct ChannelsResponse {
    pub channels: Vec<ChannelInfo>,
}

#[cw_serde]
pub struct TrappedErrorsResponse {
    pub errors: Vec<(u64, Trap)>,
}

#[cw_serde]
pub struct LpSharesResponse {
    pub lp_shares: LpCache,
}

#[cw_serde]
pub struct ConfigResponse {
    pub config: Config,
}

#[cw_serde]
pub struct LockResponse {
    pub lock: ibc_lock::Lock,
}

#[cw_serde]
pub struct IcaAddressResponse {
    pub address: String,
}

#[cw_serde]
pub struct PrimitiveSharesResponse {
    pub total: Uint128,
}

#[cw_serde]
pub struct IcaBalanceResponse {
    pub amount: Coin,
}

#[cw_serde]
pub struct IcaChannelResponse {
    pub channel: String,
}

#[cw_serde]
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
