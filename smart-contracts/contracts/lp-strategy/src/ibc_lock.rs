use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Lock {
    pub bond: IbcLock,
    pub start_unbond: IbcLock,
    pub unbond: IbcLock,
    pub recovery: IbcLock,
    pub migration: IbcLock,
}

impl Lock {
    pub fn new() -> Self {
        Lock {
            bond: IbcLock::Unlocked,
            start_unbond: IbcLock::Unlocked,
            unbond: IbcLock::Unlocked,
            recovery: IbcLock::Unlocked,
            migration: IbcLock::Unlocked,
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

    pub fn unlock_migration(mut self) -> Self {
        self.migration = IbcLock::Unlocked;
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

    pub fn lock_migration(mut self) -> Self {
        self.migration = IbcLock::Locked;
        self
    }

    // this doesnt take into account the recovery lock
    pub fn is_unlocked(&self) -> bool {
        self.bond.is_unlocked()
            && self.start_unbond.is_unlocked()
            && self.unbond.is_unlocked()
            && self.migration.is_unlocked()
    }

    // this doesnt take into account the recovery lock
    pub fn is_locked(&self) -> bool {
        self.bond.is_locked()
            || self.start_unbond.is_locked()
            || self.unbond.is_locked()
            || self.migration.is_locked()
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

// write tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock() {
        // start from unlocked
        let mut lock = Lock::new();
        assert!(lock.is_unlocked());
        assert!(!lock.is_locked());

        // lock one by one and check
        lock = lock.lock_bond();
        assert!(!lock.is_unlocked());
        assert!(lock.is_locked());

        lock = lock.lock_start_unbond();
        assert!(!lock.is_unlocked());
        assert!(lock.is_locked());

        lock = lock.lock_unbond();
        assert!(!lock.is_unlocked());
        assert!(lock.is_locked());

        // manually lock recovery
        lock.recovery = IbcLock::Locked;
        assert!(!lock.is_unlocked());
        assert!(lock.is_locked());

        lock = lock.lock_migration();
        assert!(!lock.is_unlocked());
        assert!(lock.is_locked());

        // all should be locked
        assert!(lock.bond.is_locked());
        assert!(lock.start_unbond.is_locked());
        assert!(lock.unbond.is_locked());
        assert!(lock.recovery.is_locked());
        assert!(lock.migration.is_locked());

        // none should be unlocked
        assert!(!lock.bond.is_unlocked());
        assert!(!lock.start_unbond.is_unlocked());
        assert!(!lock.unbond.is_unlocked());
        assert!(!lock.recovery.is_unlocked());
        assert!(!lock.migration.is_unlocked());

        // unlock one by one and check
        lock = lock.unlock_bond();
        assert!(!lock.is_unlocked());
        assert!(lock.is_locked());

        lock = lock.unlock_start_unbond();
        assert!(!lock.is_unlocked());
        assert!(lock.is_locked());

        lock = lock.unlock_unbond();
        assert!(!lock.is_unlocked());
        assert!(lock.is_locked());

        // manually unlock recovery
        lock.recovery = IbcLock::Unlocked;
        assert!(!lock.is_unlocked());
        assert!(lock.is_locked());

        lock = lock.unlock_migration();
        assert!(lock.is_unlocked());
        assert!(!lock.is_locked());

        // all should be unlocked
        assert!(lock.bond.is_unlocked());
        assert!(lock.start_unbond.is_unlocked());
        assert!(lock.unbond.is_unlocked());
        assert!(lock.recovery.is_unlocked());
        assert!(lock.migration.is_unlocked());

        // none should be locked
        assert!(!lock.bond.is_locked());
        assert!(!lock.start_unbond.is_locked());
        assert!(!lock.unbond.is_locked());
        assert!(!lock.recovery.is_locked());
        assert!(!lock.migration.is_locked());
    }
}
