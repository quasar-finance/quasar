use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use mars_owner::Owner;
use osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute;

#[cw_serde]
pub struct RecipientInfo {
    pub address: Addr,
    pub denom: String,
}

pub const PATHS: Map<(String, String, u64), Vec<SwapAmountInRoute>> = Map::new("paths");
pub const RECIPIENT_INFO: Item<RecipientInfo> = Item::new("recipient");
pub const OWNER: Owner = Owner::new("owner");
