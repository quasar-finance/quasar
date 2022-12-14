use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw_controllers::Claims;
use cw_storage_plus::Item;
use cw_utils::Duration;

use crate::msg::PrimitiveStrategy;

// constants
pub const FALLBACK_RATIO: Decimal = Decimal::one();

// reply ids
pub const STRATEGY_INIT_ID: u64 = 80085;

// version info for migration info
pub const CONTRACT_NAME: &str = "crates.io:cw20-staking";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


pub const CLAIMS: Claims = Claims::new("claims");

/// Investment info is fixed at instantiation, and is used to control the function of the contract
#[cw_serde]
pub struct InvestmentInfo {
    /// Owner created the contract and takes a cut
    pub owner: Addr,
    /// This is the denomination we can stake (and only one we accept for payments)
    pub bond_denom: String,
    /// This is the minimum amount we will pull out to reinvest, as well as a minimum
    /// that can be unbonded (to avoid needless staking tx)
    pub min_withdrawal: Uint128,
    /// this is the array of primitives that this vault will subscribe to
    pub primitives: Vec<PrimitiveStrategy>,
}

/// Supply is dynamic and tracks the current supply of staked and ERC20 tokens.
#[cw_serde]
#[derive(Default)]
pub struct Supply {
    /// issued is how many derivative tokens this contract has issued
    pub issued: Uint128,
    /// bonded is how many native tokens exist bonded to the validator
    pub bonded: Uint128,
    /// claims is how many tokens need to be reserved paying back those who unbonded
    pub claims: Uint128,
}

pub const INVESTMENT: Item<InvestmentInfo> = Item::new("invest");
pub const TOTAL_SUPPLY: Item<Supply> = Item::new("total_supply");


#[cw_serde]
pub struct Primitive {
    pub address: String,
    pub weight: Decimal,
}

pub const PRIMITIVES: Item<Vec<Primitive>> = Item::new("primitives");