use apollo_cw_asset::AssetInfo;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::Item;

pub const ADMIN_ADDRESS: Item<Addr> = Item::new("admin_address"); // aliceaddress
pub const VAULT_CONFIG: Item<Config> = Item::new("vault_config");
pub const BASE_TOKEN: Item<AssetInfo> = Item::new("base_token");

/// Base config struct for the contract.
#[cw_serde]
pub struct Config {
    /// Percentage of profit to be charged as performance fee
    pub performance_fee: Decimal,
    /// Account to receive fee payments
    pub treasury: Addr,
}
