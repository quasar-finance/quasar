use osmosis_std::types::osmosis::gamm::v1beta1::QueryCalcJoinPoolSharesResponse;

use quasar_types::ibc::ChannelInfo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

use cosmwasm_std::{Addr, IbcAcknowledgement, StdError, StdResult, Timestamp, Uint128};
use cw_storage_plus::{Deque, Item, Key, KeyDeserialize, Map, Prefixer, PrimaryKey};

use crate::{
    bond::Bond,
    error::{ContractError, Trap},
    helpers::{IbcMsgKind, SubMsgKind},
    ibc_lock::Lock,
    start_unbond::StartUnbond,
};

pub const RETURN_SOURCE_PORT: &str = "transfer";
pub const IBC_TIMEOUT_TIME: u64 = 7200;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    // The lock period is the amount of time we lock tokens on Osmosis
    pub lock_period: u64,
    pub pool_id: u64,
    // pool_denom is the denom of the gamm pool on osmosis; eg gamm/pool/1
    pub pool_denom: String,
    // the base denom of the pool on osmosis
    pub base_denom: String,
    //  the quote denom is the "other" side of the pool we deposit tokens in
    pub quote_denom: String,
    // the denom on the Quasar chain
    pub local_denom: String,
    // the transfer channel to transfer funds to Osmosis
    pub transfer_channel: String,
    // the channel for sending tokens back from the counterparty chain to quasar chain
    pub return_source_channel: String,
    // expected_connection id on which the primitive should function
    pub expected_connection: String,
}

// TODO remove the need for ADMIN

// the ADMIN in this case is the person allowed to deposit into the contract
// this is set to the first depositor
pub(crate) const ADMIN: Item<Addr> = Item::new("admin");
pub(crate) const DEPOSITOR: Item<Addr> = Item::new("depositor");

pub(crate) const CONFIG: Item<Config> = Item::new("config");
// IBC related state items
pub(crate) const REPLIES: Map<u64, SubMsgKind> = Map::new("replies");
// RECOVERY_ACK contains ibc acknowledgements, these packets might be needed for recovery from errors
pub(crate) const RECOVERY_ACK: Map<u64, IbcAcknowledgement> = Map::new("recovery_ack");

// true when a packet has timed out and the ica channel needs to be closed and a new channel needs to be opened
pub(crate) const TIMED_OUT: Item<bool> = Item::new("timed_out");
// Currently we only support one ICA channel to a single destination
pub(crate) const ICA_CHANNEL: Item<String> = Item::new("ica_channel");
// We also support one ICQ channel to Osmosis at the moment
pub(crate) const ICQ_CHANNEL: Item<String> = Item::new("icq_channel");

pub(crate) const CHANNELS: Map<String, ChannelInfo> = Map::new("channels");
pub(crate) const OLD_PENDING_ACK: Map<u64, IbcMsgKind> = Map::new("pending_acks");

pub(crate) const NEW_PENDING_ACK: Map<(u64, String), IbcMsgKind> = Map::new("new_pending_acks");
// The map to store trapped errors,
pub(crate) const TRAPS: Map<u64, Trap> = Map::new("traps");

// all vault related state items
pub(crate) const IBC_LOCK: Item<Lock> = Item::new("lock");
pub(crate) const PENDING_BOND_QUEUE: Deque<Bond> = Deque::new("pending_bond_queue");
pub(crate) const BOND_QUEUE: Deque<Bond> = Deque::new("bond_queue");
pub(crate) const START_UNBOND_QUEUE: Deque<StartUnbond> = Deque::new("start_unbond_queue");
pub(crate) const UNBOND_QUEUE: Deque<Unbond> = Deque::new("unbond_queue");
// the amount of LP shares that the contract has entered into the pool
pub(crate) const LP_SHARES: Item<LpCache> = Item::new("lp_shares");

// the latest known ica balance
pub(crate) const TOTAL_VAULT_BALANCE: Item<Uint128> = Item::new("total_vault_balance");

// TODO we probably want to change this to an OngoingDeposit
pub(crate) const BONDING_CLAIMS: Map<(&Addr, &str), Uint128> = Map::new("bonding_claims");

// our c
pub(crate) const PENDING_UNBONDING_CLAIMS: Map<(Addr, String), Unbond> =
    Map::new("unbonding_claims");
pub(crate) const UNBONDING_CLAIMS: Map<(Addr, String), Unbond> = Map::new("unbonding_claims");
// TODO make key borrowed
pub(crate) const SHARES: Map<Addr, Uint128> = Map::new("shares");
// the lock id on osmosis, for each combination of denom and lock duration, only one lock id should exist on osmosis
pub(crate) const OSMO_LOCK: Item<u64> = Item::new("osmo_lock");
// the returning transfer we can expect and their exact amount
pub(crate) const RETURNING: Map<u64, Uint128> = Map::new("returning");
// TODO, do we remove this state item? is it needed?
// whatever the above todo item is, does not apply to the following
// we save the queried simulate join swap during ICQ so we can read it right before bond join
pub(crate) const SIMULATED_JOIN_RESULT: Item<Uint128> = Item::new("simulated_join_result");
// we save the amount that went into the QueryCalcJoinPool, so we can scale up the slippage amount if more deposits come
pub(crate) const SIMULATED_JOIN_AMOUNT_IN: Item<Uint128> = Item::new("simulated_join_amount");
// we also save the queried simulate exit swap during ICQ so we can read it right before unbond exit
pub(crate) const SIMULATED_EXIT_RESULT: Item<Uint128> = Item::new("simulated_exit_result");
// CLAIMABLE_FUNDS is the amount of funds claimable by a certain address, either
pub(crate) const CLAIMABLE_FUNDS: Map<(Addr, FundPath), Uint128> = Map::new("claimable_funds");

impl PrimaryKey<'_> for FundPath {
    type Prefix = Addr;

    type SubPrefix = ();

    type Suffix = u8;

    type SuperSuffix = Self;

    fn key(&self) -> Vec<Key> {
        // this is a bit yikes but fuck it
        match self {
            FundPath::Bond { id } => vec![Key::Val8([0]), Key::Ref(id.as_bytes())],
            FundPath::Unbond { id } => vec![Key::Val8([1]), Key::Ref(id.as_bytes())],
        }
    }
}

impl KeyDeserialize for FundPath {
    type Output = FundPath;

    #[inline(always)]
    fn from_vec(value: Vec<u8>) -> StdResult<Self::Output> {
        if value[0] == 0 {
            Ok(FundPath::Bond {
                id: String::from_utf8(value[1..].to_vec()).map_err(|err| {
                    StdError::InvalidUtf8 {
                        msg: err.to_string(),
                    }
                })?,
            })
        } else if value[0] == 1 {
            Ok(FundPath::Unbond {
                id: String::from_utf8(value[1..].to_vec()).map_err(|err| {
                    StdError::InvalidUtf8 {
                        msg: err.to_string(),
                    }
                })?,
            })
        } else {
            Err(StdError::SerializeErr {
                source_type: "key-de".to_string(),
                msg: "enum variant not found".to_string(),
            })
        }
    }
}

impl Prefixer<'_> for FundPath {
    fn prefix(&self) -> Vec<Key> {
        match self {
            FundPath::Bond { id } => vec![Key::Ref(id.as_bytes())],
            FundPath::Unbond { id } => vec![Key::Ref(id.as_bytes())],
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FundPath {
    Bond { id: String },
    Unbond { id: String },
}

impl Display for FundPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct LpCache {
    // the amount of locked shares we currently have
    pub locked_shares: Uint128,
    // the amount of unlocked share we have for withdrawing
    pub w_unlocked_shares: Uint128,
    // the amount unlocked shares we have for depositing
    pub d_unlocked_shares: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Unbond {
    pub lp_shares: Uint128,
    pub unlock_time: Timestamp,
    pub attempted: bool,
    pub owner: Addr,
    pub id: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Eq)]
#[serde(rename_all = "snake_case")]
pub struct PendingSingleUnbond {
    pub lp_shares: Uint128,
    pub primitive_shares: Uint128,
    pub owner: Addr,
    pub id: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PendingBond {
    // the bonds of the original calls
    pub bonds: Vec<OngoingDeposit>,
}

impl PendingBond {
    pub fn update_raw_amount_to_lp(&mut self, total_lp: Uint128) -> Result<(), ContractError> {
        let mut total = Uint128::zero();
        for p in self.bonds.iter() {
            match p.raw_amount {
                crate::state::RawAmount::LocalDenom(val) => total = total.checked_add(val)?,
                crate::state::RawAmount::LpShares(_) => unimplemented!(),
            }
        }
        for p in self.bonds.iter_mut() {
            match p.raw_amount {
                // amount of lp shares = val * total_lp / total
                crate::state::RawAmount::LocalDenom(val) => {
                    p.raw_amount =
                        RawAmount::LpShares(val.checked_mul(total_lp)?.checked_div(total)?)
                }
                crate::state::RawAmount::LpShares(_) => unimplemented!(),
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Eq)]
#[serde(rename_all = "snake_case")]
pub struct Claim {
    amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OngoingDeposit {
    pub claim_amount: Uint128, // becomes shares later
    pub raw_amount: RawAmount,
    pub owner: Addr,
    pub bond_id: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RawAmount {
    LocalDenom(Uint128),
    LpShares(Uint128),
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn keys_work() {
        let bond = FundPath::Bond {
            id: "our-id-here".to_string(),
        };
        let keys: Vec<u8> = bond
            .key()
            .iter()
            .flat_map(|k| k.as_ref().iter().copied())
            .collect();
        let value = FundPath::from_vec(keys).unwrap();
        assert_eq!(bond, value)
    }

    #[test]
    fn test_update_raw_amount_to_lp() {
        let mut pending = PendingBond {
            bonds: vec![
                OngoingDeposit {
                    claim_amount: Uint128::new(100),
                    raw_amount: RawAmount::LocalDenom(Uint128::new(1000)),
                    owner: Addr::unchecked("address"),
                    bond_id: "fake".to_string(),
                },
                OngoingDeposit {
                    claim_amount: Uint128::new(99),
                    raw_amount: RawAmount::LocalDenom(Uint128::new(999)),
                    owner: Addr::unchecked("address"),
                    bond_id: "fake".to_string(),
                },
                OngoingDeposit {
                    claim_amount: Uint128::new(101),
                    raw_amount: RawAmount::LocalDenom(Uint128::new(1000)),
                    owner: Addr::unchecked("address"),
                    bond_id: "fake".to_string(),
                },
            ],
        };
        pending.update_raw_amount_to_lp(Uint128::new(300)).unwrap();
        assert_eq!(
            pending.bonds[0].raw_amount,
            RawAmount::LpShares(Uint128::new(100))
        );
        assert_eq!(
            pending.bonds[1].raw_amount,
            RawAmount::LpShares(Uint128::new(99))
        );
        // because we use integer division and relatively low values, this case us 100
        assert_eq!(
            pending.bonds[2].raw_amount,
            RawAmount::LpShares(Uint128::new(100))
        )
    }
}
