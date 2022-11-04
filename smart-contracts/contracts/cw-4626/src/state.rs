use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use cosmwasm_std::Uint128;
use cw_storage_plus::Item;
use quasar_types::curve::{CurveType, DecimalPlaces};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct VaultInfo {
    // reserve_denom is the denomination accepted by this vault. If the accepted token should be
    // a cw20-token, one should wrap the token using the tokenfactory and the wrapping contract
    pub reserve_denom: String,
    // total_supply is the total supply of shares this contract
    pub total_supply: Uint128,
    // the decimals of the vault supply and reserve
    pub decimals: DecimalPlaces,
}

pub const VAULT_INFO: Item<VaultInfo> = Item::new("vault_info");
pub const VAULT_CURVE: Item<CurveType> = Item::new("vault_curve");
