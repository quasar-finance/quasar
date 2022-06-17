use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use quasar_traits::traits::ShareDistributor;
use quasar_types::curve::{CurveType, DecimalPlaces};

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
    // TODO see if we can change supply and reserve for an iterator over OUTSTANDING_SHARES and VAULT_RESERVES
    // the current outstanding supply
    pub supply: Uint128,
    // the current total reserve
    pub reserve: Uint128,
    // the decimals of the vault supply and reserve
    pub decimals: DecimalPlaces,
}


pub const VAULT_INFO: Item<VaultInfo> = Item::new("vault_info");
pub const VAULT_CURVE: Item<CurveType> = Item::new("vault_curve");
// TODO I think OUTSTANDING_SHARES and VAULT_RESERVES are deprecated, see if we can actually remove them
pub const OUTSTANDING_SHARES: Map<String, Uint128> = Map::new("outstanding_shares");
pub const VAULT_RESERVES: Map<&Addr, Uint128> = Map::new("vault_reserves");
