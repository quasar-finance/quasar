use std::collections::HashMap;

use cosmwasm_std::{Addr, Coin, StdResult, Uint128};

use quasar_types::ibc::ChannelInfo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    error::Trap,
    helpers::IbcMsgKind,
    ibc_lock,
    state::{Config, Unbond},
};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {
    pub lock_period: u64,
    pub pool_id: u64,
    pub pool_denom: String,
    pub local_denom: String,
    pub base_denom: String,
    pub quote_denom: String,
    pub transfer_channel: String,
    pub return_source_channel: String,
}

impl InstantiateMsg {
    pub fn validate(&self) -> StdResult<()> {
        Ok(())
    }
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ListBondingClaimsResponse {
    pub bonds: HashMap<(Addr, String), Uint128>,
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
    pub lp_shares: Uint128,
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
    // accept a dispatched transfer from osmosis
    AcceptReturningFunds {
        id: u64,
    },
    // all execute msges below are used for testing and should be removed before productions
    TransferJoinLock {
        channel: String,
        to_address: String,
    },
    DepositAndLockTokens {
        pool_id: u64,
        amount: Uint128,
        denom: String,
        share_out_min_amount: Uint128,
    },
    ReturnTransfer {
        amount: Uint128,
    },
}
