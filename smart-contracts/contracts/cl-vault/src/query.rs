use crate::{
    concentrated_liquidity::get_position,
    error::ContractResult,
    state::{
        PoolConfig, ADMIN_ADDRESS, METADATA, POOL_CONFIG, POSITION, SHARES, USER_REWARDS,
        VAULT_DENOM,
    },
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Coin, Deps, Env, Uint128};
use cw_vault_multi_standard::VaultInfoResponse;
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
pub struct PoolResponse {
    pub pool_config: PoolConfig,
}

#[cw_serde]
pub struct PositionResponse {
    pub position_ids: Vec<u64>,
}

#[cw_serde]
pub struct UserBalanceResponse {
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
    total: Uint128,
}

pub fn query_metadata(deps: Deps) -> ContractResult<MetadataResponse> {
    let metadata = METADATA.load(deps.storage)?;
    let vault_denom = VAULT_DENOM.load(deps.storage)?;
    let total_supply = BankQuerier::new(&deps.querier)
        .supply_of(vault_denom.clone())?
        .amount
        .unwrap()
        .amount
        .parse::<u128>()?
        .into();
    let admin = ADMIN_ADDRESS.load(deps.storage)?.to_string();

    Ok(MetadataResponse {
        thesis: metadata.thesis,
        name: metadata.name,
        total_supply,
        symbol: vault_denom,
        decimals: 6,
        admin,
    })
}

pub fn query_info(deps: Deps) -> ContractResult<VaultInfoResponse> {
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let vault_token = VAULT_DENOM.load(deps.storage)?;
    Ok(VaultInfoResponse {
        tokens: vec![pool_config.token0, pool_config.token1],
        vault_token,
    })
}

pub fn query_pool(deps: Deps) -> ContractResult<PoolResponse> {
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    Ok(PoolResponse { pool_config })
}

pub fn query_position(deps: Deps) -> ContractResult<PositionResponse> {
    let position_id = POSITION.load(deps.storage)?.position_id;
    Ok(PositionResponse {
        position_ids: vec![position_id],
    })
}
pub fn query_user_balance(deps: Deps, user: String) -> ContractResult<UserBalanceResponse> {
    let balance = SHARES.load(deps.storage, deps.api.addr_validate(&user)?)?;
    Ok(UserBalanceResponse { balance })
}

pub fn query_user_rewards(deps: Deps, user: String) -> ContractResult<UserRewardsResponse> {
    let rewards = USER_REWARDS
        .load(deps.storage, deps.api.addr_validate(&user)?)?
        .into_coins();
    Ok(UserRewardsResponse { rewards })
}

pub fn query_total_assets(deps: Deps, env: Env) -> ContractResult<TotalAssetsResponse> {
    let position = get_position(deps.storage, &deps.querier, &env)?;
    let pool = POOL_CONFIG.load(deps.storage)?;
    Ok(TotalAssetsResponse {
        token0: position
            .asset0
            .map(|c| c.try_into().unwrap())
            .unwrap_or(coin(0, pool.token0)),
        token1: position
            .asset1
            .map(|c| c.try_into().unwrap())
            .unwrap_or(coin(0, pool.token1)),
    })
}

pub fn query_total_vault_token_supply(deps: Deps) -> ContractResult<TotalVaultTokenSupplyResponse> {
    let bq = BankQuerier::new(&deps.querier);
    let vault_denom = VAULT_DENOM.load(deps.storage)?;
    let total = bq
        .supply_of(vault_denom)?
        .amount
        .unwrap()
        .amount
        .parse::<u128>()?
        .into();
    Ok(TotalVaultTokenSupplyResponse { total })
}
