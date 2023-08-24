use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Addr, Binary, Coin, Deps, Uint128};
use cw_vault_multi_standard::VaultInfoResponse;

use crate::{
    error::ContractResult,
    state::{PoolConfig, LOCKED_SHARES, POOL_CONFIG, POSITION, USER_REWARDS, VAULT_DENOM},
};

#[cw_serde]
pub struct PoolResponse {
    pub pool_config: PoolConfig,
}

#[cw_serde]
pub struct PositionResponse {
    pub position_id: u64,
}

#[cw_serde]
pub struct UserBalanceResponse {
    pub balance: Uint128,
}

#[cw_serde]
pub struct UserRewardsResponse {
    pub rewards: Vec<Coin>,
}

pub fn query_info(deps: Deps) -> ContractResult<Binary> {
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let vault_token = VAULT_DENOM.load(deps.storage)?;
    Ok(to_binary(&VaultInfoResponse {
        tokens: vec![pool_config.token0, pool_config.token1],
        vault_token,
    })?)
}

pub fn query_pool(deps: Deps) -> ContractResult<Binary> {
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    Ok(to_binary(&PoolResponse { pool_config })?)
}

pub fn query_position(deps: Deps) -> ContractResult<Binary> {
    let position_id = POSITION.load(deps.storage)?.position_id;
    Ok(to_binary(&PositionResponse { position_id })?)
}
pub fn query_user_balance(deps: Deps, user: Addr) -> ContractResult<Binary> {
    let balance = LOCKED_SHARES.load(deps.storage, user)?;
    Ok(to_binary(&UserBalanceResponse { balance })?)
}

pub fn query_user_rewards(deps: Deps, user: Addr) -> ContractResult<Binary> {
    let rewards = USER_REWARDS.load(deps.storage, user)?.into_coins();
    Ok(to_binary(&UserRewardsResponse { rewards })?)
}
