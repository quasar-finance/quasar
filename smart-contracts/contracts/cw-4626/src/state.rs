use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use quasar_traits::traits::ShareDistributor;

use serde::de::DeserializeOwned;
use share_distributor::single_token::SingleToken;


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct VaultInfo {
    // reserve_denom is the denomination accepted by this vault. If the accepted token should be
    // a cw20-token, one should wrap the token using the tokenfactory and the wrapping contract
    pub reserve_denom: String,
    // total_supply is the total supply of shares this contract
    pub total_supply: Uint128,
}


// we wrap our generic share distributor trait in a struct. This way, people writing their own
// distributor either do some large changes, or implement their own version of the ShareDistributor
// trait and get all the "code hints" we leave along the way
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Distributor<T: ShareDistributor + Clone + PartialEq + JsonSchema + Debug> {
    pub dist: T,
}

pub const VAULT_INFO: Item<VaultInfo> = Item::new("vault_info");
pub const OUTSTANDING_SHARES: Map<String, Uint128> = Map::new("outstanding_shares");
pub const VAULT_DISTRIBUTOR: Item<Distributor<SingleToken>> = Item::new("vault_distributor");
pub const VAULT_RESERVES: Map<&Addr, String> = Map::new("vault_balances");
