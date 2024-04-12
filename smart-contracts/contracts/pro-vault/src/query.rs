use crate::error::ContractResult;
use crate::ContractError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Coin, Decimal, Deps, Env, Uint128};
use osmosis_std::types::cosmos::bank::v1beta1::BankQuerier;

#[cw_serde]
pub struct MetadataResponse {
    // thesis -> actual metadata
    pub thesis: String,
    // name -> actual metadata
    pub name: String,
    // total_supply -> from denom
    pub total_supply: Uint128,
    // symbol -> tokenfactory denom
    pub symbol: String,
    // decimals -> hardcoded since native denom
    pub decimals: u8,
    // owner -> admin
    pub admin: String,
}

#[cw_serde]
pub struct PositionResponse {
    pub position_ids: Vec<u64>,
}

#[cw_serde]
pub struct AssetsBalanceResponse {
    pub balances: Vec<Coin>,
}

#[cw_serde]
pub struct UserSharesBalanceResponse {
    pub balance: Uint128,
}

#[cw_serde]
pub struct UserRewardsResponse {
    pub rewards: Vec<Coin>,
}

#[cw_serde]
pub struct TotalAssetsResponse {
    pub token0: Coin,
    pub token1: Coin,
}

#[cw_serde]
pub struct RangeAdminResponse {
    pub address: String,
}

#[cw_serde]
pub struct TotalVaultTokenSupplyResponse {
    pub total: Uint128,
}

#[cw_serde]
pub struct VerifyTickCacheResponse {
    pub result: Result<(), i64>,
}

#[cw_serde]
pub struct DexRouterResponse {
    pub dex_router: String,
}
