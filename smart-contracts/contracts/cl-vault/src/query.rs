use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Binary, Deps};
use cw_vault_multi_standard::VaultInfoResponse;

use crate::{
    error::ContractResult,
    state::{PoolConfig, POOL_CONFIG, VAULT_DENOM},
};

#[cw_serde]
pub struct PoolResponse {
    pub pool_config: PoolConfig,
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
