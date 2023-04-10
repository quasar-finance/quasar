use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, IbcPacketAckMsg, StdResult, Uint128};

use quasar_types::ibc::ChannelInfo;

use crate::{
    bond::Bond,
    error::Trap,
    helpers::{IbcMsgKind, SubMsgKind},
    ibc_lock,
    start_unbond::StartUnbond,
    state::{Config, LpCache, Unbond},
};

#[cw_serde]
pub struct InstantiateMsg {
    pub lock_period: u64,
    pub pool_id: u64,       // 2
    pub pool_denom: String, // gamm/pool/2
    // if setup correctly, local_denom on quasar == base_denom on osmosis
    pub local_denom: String, // ibc/ED07
    pub base_denom: String,  // uosmo
    pub quote_denom: String, // uatom
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
pub struct MigrateMsg {}

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
    #[returns(OsmoLockResponse)]
    OsmoLock {},
    #[returns(SimulatedJoinResponse)]
    SimulatedJoin {},
    #[returns(GetQueuesResponse)]
    GetQueues {},
}

#[cw_serde]
pub struct GetQueuesResponse {
    pub pending_bond_queue: Vec<Bond>,
    pub bond_queue: Vec<Bond>,
    pub start_unbond_queue: Vec<StartUnbond>,
    pub unbond_queue: Vec<Unbond>,
}

#[cw_serde]
pub struct SimulatedJoinResponse {
    pub amount: Option<Uint128>,
    pub result: Option<Uint128>,
}

#[cw_serde]
pub struct OsmoLockResponse {
    pub lock_id: u64,
}

#[cw_serde]
pub struct ListBondingClaimsResponse {
    pub bonds: HashMap<Addr, (String, Uint128)>,
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
    pub pending: HashMap<String, IbcMsgKind>,
}

#[cw_serde]
pub struct ListUnbondingClaimsResponse {
    pub unbonds: HashMap<Addr, (String, Unbond)>,
}

#[cw_serde]
pub struct UnbondingClaimResponse {
    pub unbond: Option<Unbond>,
}

#[cw_serde]
pub struct ChannelsResponse {
    pub channels: Vec<ChannelInfo>,
}

#[cw_serde]
pub struct TrappedErrorsResponse {
    pub errors: HashMap<String, Trap>,
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
pub enum UnlockOnly {
    Bond,
    StartUnbond,
    Unbond,
    Migration,
}

impl Display for UnlockOnly {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UnlockOnly::Bond => write!(f, "bond"),
            UnlockOnly::StartUnbond => write!(f, "start_unbond"),
            UnlockOnly::Unbond => write!(f, "unbond"),
            UnlockOnly::Migration => write!(f, "migration"),
        }
    }
}

#[cw_serde]
pub enum LockOnly {
    Bond,
    StartUnbond,
    Unbond,
    Migration,
}

impl Display for LockOnly {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LockOnly::Bond => write!(f, "bond"),
            LockOnly::StartUnbond => write!(f, "start_unbond"),
            LockOnly::Unbond => write!(f, "unbond"),
            LockOnly::Migration => write!(f, "migration"),
        }
    }
}

#[cw_serde]
pub enum ExecuteMsg {
    Bond {
        id: String,
    },
    StartUnbond {
        id: String,
        share_amount: Uint128,
    },
    Unbond {
        id: String,
    },
    SetDepositor {
        depositor: String,
    },
    // accept a dispatched transfer from osmosis
    AcceptReturningFunds {
        id: u64,
    },
    // try to close a channel where a timout occured
    CloseChannel {
        channel_id: String,
    },
    Ack {
        ack: IbcPacketAckMsg,
    },
    TryIcq {},
    Unlock {
        unlock_only: UnlockOnly,
    },
    Lock {
        lock_only: LockOnly,
    },
    ManualTimeout {
        seq: u64,
        channel: String,
        should_unlock: bool,
    },
}
