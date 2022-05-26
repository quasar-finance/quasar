use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Order, StdResult, Storage, Uint128};
use quasar_traits::traits::ShareDistributor;
use cw_storage_plus::{Bound, Item, Map};

use cw20::{AllowanceResponse, Balance, Cw20Coin, Logo, MarketingInfoResponse};
use serde::de::DeserializeOwned;
use share_distributor::single_token::SingleToken;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
    pub mint: Option<MinterData>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct VaultInfo {
    pub vault_whitelist: Vec<Addr>,
}


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MinterData {
    pub minter: Addr,
    /// cap is how many more tokens can be issued by the minter
    pub cap: Option<Uint128>,
}

impl TokenInfo {
    pub fn get_cap(&self) -> Option<Uint128> {
        self.mint.as_ref().and_then(|v| v.cap)
    }
}

// we wrap our generic share distributor trait in a struct. This way, people writing their own
// distributor either do some large changes, or implement their own version of the ShareDistributor
// trait and get all the "code hints" we leave along the way
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(bound = "")]
pub struct Distributor<'a, T: ShareDistributor + Serialize + Deserialize<'a> + Clone + PartialEq + JsonSchema + Debug>
{
    #[serde(bound = "T: Deserialize"<'a> + Serialize)]
    pub dist: T,
    pub phantom: PhantomData<&'a T>,
}

pub const TOKEN_INFO: Item<TokenInfo> = Item::new("token_info");
pub const VAULT_INFO: Item<VaultInfo> = Item::new("vault_info");
pub const VAULT_DISTRIBUTOR: Item<Distributor<SingleToken>> = Item::new("vault_distributor");
// TODO change this to accept native tokens
pub const VAULT_BALANCES: Map<&Addr, String> = Map::new("vault_balances");
pub const MARKETING_INFO: Item<MarketingInfoResponse> = Item::new("marketing_info");
pub const LOGO: Item<Logo> = Item::new("logo");
pub const BALANCES: Map<&Addr, Uint128> = Map::new("balance");
pub const ALLOWANCES: Map<(&Addr, &Addr), AllowanceResponse> = Map::new("allowance");
