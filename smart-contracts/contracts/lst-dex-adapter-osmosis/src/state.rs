use abstract_app::objects::PoolAddress;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};
use cw_asset::AssetInfo;
use cw_storage_plus::Item;

#[cw_serde]
pub struct State {
    pub lst_adapter: Addr,
    pub dex: String,
    pub offer_asset: AssetInfo,
    pub receive_asset: AssetInfo,
    pub margin: Decimal,
    pub pool: PoolAddress,
}

pub const STATE: Item<State> = Item::new("config");
pub const RECIPIENT: Item<Option<Addr>> = Item::new("recipient");
