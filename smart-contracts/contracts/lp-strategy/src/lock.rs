use cosmwasm_std::{Addr, Storage, Uint128};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{error::ContractError, state::LOCK_QUEUE};

/// Lock describes the current state of the contract
/// Upon locking the contract, all current deposits and withdraws are going to be handled, Incoming withdraws are gathered once again gathered into a queue.
/// Once the contract unlocks, if the queue has any deposits and/or withdraws, the contract locks and starts handling all current queries
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Lock {
    Locked,
    Unlocked,
}

// TODO rename this
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DWType {
    Withdraw(Withdraw),
    Deposit(Deposit),
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Deposit {
    pub amount: Uint128,
    pub owner: Addr,
}

impl Deposit {
    fn validate(&self) -> Result<(), ContractError> {
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Withdraw {
    amount: Uint128,
    owner: Addr,
}

impl Withdraw {
    fn validate(&self) -> Result<(), ContractError> {
        Ok(())
    }
}

/// enqueue is a wrapper around our LOCK_QUEUE storage
pub fn enqueue(storage: &mut dyn Storage, item: DWType) -> Result<(), ContractError> {
    match &item {
        DWType::Withdraw(withdraw) => withdraw.validate()?,
        DWType::Deposit(deposit) => deposit.validate()?,
    }
    Ok(LOCK_QUEUE.push_back(storage, &item)?)
}
