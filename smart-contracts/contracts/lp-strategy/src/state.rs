use quasar_types::ibc::ChannelInfo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Deque, Item, Map};

use crate::{
    error::{ContractError, Trap},
    helpers::IbcMsgKind,
    lock::{DWType, Lock},
};

pub const RETURN_SOURCE_PORT: &'static str = "transfer";

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct Config {
    // The lock period is the amount of time we lock tokens on Osmosis
    pub lock_period: Uint128,
    pub unbonding_period: Uint128,
    pub pool_id: u64,
    // pool_denom is the denom of the gamm pool on osmosis; eg gamm/pool/1
    pub pool_denom: String,
    // the base denom of the pool on osmosis
    pub base_denom: String,
    //  the quote denom is the "other" side of the pool we deposit tokens in
    pub quote_denom: String,
    // the denom on the Quasar chain
    pub local_denom: String,
    // the channel for sending tokens back from the counterparty chain to quasar chain
    pub return_source_channel: String,
}

pub(crate) const CONFIG: Item<Config> = Item::new("tmp");

// IBC related state items
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
pub(crate) const TRAPS: Map<u64, Trap> = Map::new("traps");

// all vault related state items

pub(crate) const LOCK: Item<Lock> = Item::new("lock");
pub(crate) const LOCK_QUEUE: Deque<DWType> = Deque::new("lock_queue");
// the amount of LP shares that the contract has entered into the pool
pub(crate) const LP_SHARES: Item<Uint128> = Item::new("lp_shares");

// TODO we probably want to change this to an OngoingDeposit
pub(crate) const CLAIMS: Map<Addr, Uint128> = Map::new("claims");
pub(crate) const SHARES: Map<Addr, Uint128> = Map::new("shares");

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PendingAck {
    // the ibc msg of the sequence number
    pub kind: IbcMsgKind,
    // the deposits of the original calls
    pub deposits: Vec<OngoingDeposit>,
}

impl PendingAck {
    pub fn update_kind(&mut self, kind: IbcMsgKind) {
        self.kind = kind;
    }

    pub fn update_raw_amount_to_lp(&mut self, total_lp: Uint128) -> Result<(), ContractError> {
        let mut total = Uint128::zero();
        for p in self.deposits.iter() {
            match p.raw_amount {
                crate::state::RawAmount::LocalDenom(val) => total = total.checked_add(val)?,
                crate::state::RawAmount::LpShares(_) => unimplemented!(),
            }
        }
        for p in self.deposits.iter_mut() {
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

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Claim {
    amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OngoingDeposit {
    pub claim_amount: Uint128, // becomes shares later
    pub raw_amount: RawAmount,
    pub owner: Addr,
    pub bond_id: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RawAmount {
    LocalDenom(Uint128),
    LpShares(Uint128),
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_update_raw_amount_to_lp() {
        let mut pending = PendingAck {
            kind: IbcMsgKind::Transfer,
            deposits: vec![
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
            pending.deposits[0].raw_amount,
            RawAmount::LpShares(Uint128::new(100))
        );
        assert_eq!(
            pending.deposits[1].raw_amount,
            RawAmount::LpShares(Uint128::new(99))
        );
        // because we use integer division and relatively low values, this case us 100
        assert_eq!(
            pending.deposits[2].raw_amount,
            RawAmount::LpShares(Uint128::new(100))
        )
    }
}
