use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Lock {
    pub bond: IbcLock,
    pub start_unbond: IbcLock,
    pub unbond: IbcLock,
    pub recovery: IbcLock,
}

impl Lock {
    pub fn new() -> Self {
        Lock {
            bond: IbcLock::Unlocked,
            start_unbond: IbcLock::Unlocked,
            unbond: IbcLock::Unlocked,
            recovery: IbcLock::Unlocked,
        }
    }

    pub fn unlock_bond(mut self) -> Self {
        self.bond = IbcLock::Unlocked;
        self
    }

    pub fn unlock_start_unbond(mut self) -> Self {
        self.start_unbond = IbcLock::Unlocked;
        self
    }

    pub fn unlock_unbond(mut self) -> Self {
        self.unbond = IbcLock::Unlocked;
        self
    }

    pub fn lock_bond(mut self) -> Self {
        self.bond = IbcLock::Locked;
        self
    }

    pub fn lock_start_unbond(mut self) -> Self {
        self.start_unbond = IbcLock::Locked;
        self
    }

    pub fn lock_unbond(mut self) -> Self {
        self.unbond = IbcLock::Locked;
        self
    }

    pub fn is_unlocked(&self) -> bool {
        self.bond.is_unlocked() && self.start_unbond.is_unlocked() && self.unbond.is_unlocked()
    }

    pub fn is_locked(&self) -> bool {
        self.bond.is_locked() || self.start_unbond.is_locked() || self.unbond.is_locked()
    }
}

impl Default for Lock {
    fn default() -> Self {
        Self::new()
    }
}

/// IbcLock describes the current state of the contract
/// Upon locking the contract, all current deposits and withdraws are going to be handled, Incoming withdraws are gathered once again gathered into a queue.
/// Once the contract unlocks, if the queue has any deposits and/or withdraws, the contract locks and starts handling all current queries
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum IbcLock {
    Locked,
    Unlocked,
}

impl IbcLock {
    pub fn is_unlocked(&self) -> bool {
        self == &IbcLock::Unlocked
    }

    pub fn is_locked(&self) -> bool {
        self == &IbcLock::Locked
    }
}
