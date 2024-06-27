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

// map from (offer_denom, ask_denom) keys to a vector of swap routes
pub const PATHS: Map<(String, String), Vec<Vec<SwapAmountInRoute>>> = Map::new("paths");
pub const RECIPIENT_INFO: Item<RecipientInfo> = Item::new("recipient");
pub const OWNER: Owner = Owner::new("owner");
