use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use mars_owner::Owner;

#[derive(Default)]
#[cw_serde]
pub struct IbcConfig {
    pub remote_chain: String,
    pub channel: String,
    pub revision: Option<u64>,
    pub block_offset: Option<u64>,
    pub timeout_secs: Option<u64>,
}

pub const LST_DENOM: Item<String> = Item::new("lst_denom");
pub const OWNER: Owner = Owner::new("owner");
pub const VAULT: Item<Addr> = Item::new("vault");
pub const IBC_CONFIG: Item<IbcConfig> = Item::new("ibc_config");
