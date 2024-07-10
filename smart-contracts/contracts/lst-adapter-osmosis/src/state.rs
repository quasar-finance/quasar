use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_storage_plus::Item;
use mars_owner::Owner;
use quasar_types::denoms::LstDenom;

#[derive(Default)]
#[cw_serde]
pub struct IbcConfig {
    pub channel: String,
    pub remote_chain: String,
    pub revision: Option<u64>,
    pub block_offset: Option<u64>,
    pub timeout_secs: Option<u64>,
}

#[cw_serde]
pub enum UnbondStatus {
    Unconfirmed,
    Confirmed,
}

#[cw_serde]
pub struct UnbondInfo {
    pub amount: Uint128,
    pub unbond_start: Timestamp,
    pub status: UnbondStatus,
}

// configuration
pub const LST_DENOM: Item<LstDenom> = Item::new("lst_denom");
pub const UNBOND_PERIOD_SECS: Item<u64> = Item::new("unbond_period");
pub const OWNER: Owner = Owner::new("owner");
pub const VAULT: Item<Addr> = Item::new("vault");
pub const OBSERVER: Item<Addr> = Item::new("observer");
pub const IBC_CONFIG: Item<IbcConfig> = Item::new("ibc_config");
pub const ORACLE: Item<Addr> = Item::new("stride_oracle");
// info on pending unbondings
pub const UNBONDING: Item<Vec<UnbondInfo>> = Item::new("unbonding");
// for balance tracking
pub const REDEEMED_BALANCE: Item<Uint128> = Item::new("underlying_balance");
pub const TOTAL_BALANCE: Item<Uint128> = Item::new("total_balance");
