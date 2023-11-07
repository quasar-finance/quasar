use crate::state::{Position, POSITIONS};
use crate::vault::concentrated_liquidity::get_positions;
use crate::ContractError;
use crate::{
    error::ContractResult,
    state::{PoolConfig, ADMIN_ADDRESS, METADATA, POOL_CONFIG, SHARES, USER_REWARDS, VAULT_DENOM},
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Coin, Deps, Order, OverflowError, OverflowOperation, StdError, Uint128};
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
    pub positions: Vec<Position>,
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
    pub total: Uint128,
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

pub fn query_positions(deps: Deps) -> ContractResult<PositionResponse> {
    let positions: Result<Vec<(u64, Position)>, StdError> = POSITIONS
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    Ok(PositionResponse {
        positions: positions?.into_iter().map(|(_, p)| p).collect(),
    })
}

pub fn query_user_balance(deps: Deps, user: String) -> ContractResult<UserBalanceResponse> {
    let balance = SHARES
        .may_load(deps.storage, deps.api.addr_validate(&user)?)?
        .unwrap_or(Uint128::zero());
    Ok(UserBalanceResponse { balance })
}

pub fn query_user_rewards(deps: Deps, user: String) -> ContractResult<UserRewardsResponse> {
    let rewards = USER_REWARDS
        .load(deps.storage, deps.api.addr_validate(&user)?)?
        .coins();
    Ok(UserRewardsResponse { rewards })
}

pub fn query_total_assets(deps: Deps) -> ContractResult<TotalAssetsResponse> {
    let positions = get_positions(deps.storage, &deps.querier)?;
    let pool = POOL_CONFIG.load(deps.storage)?;
    let (asset0, asset1) = positions.iter().try_fold(
        (0_u128, 0_u128),
        |(acc0, acc1), i| -> Result<(u128, u128), ContractError> {
            let asset0 =
                i.1.asset0
                    .map(|v| v.amount.parse::<u128>().unwrap())
                    .unwrap_or(0_u128);
            let asset1 =
                i.1.asset1
                    .map(|v| v.amount.parse::<u128>().unwrap())
                    .unwrap_or(0_u128);

            Ok((
                acc0.checked_add(asset0).ok_or_else(|| {
                    StdError::overflow(OverflowError::new(OverflowOperation::Add, acc0, asset0))
                })?,
                acc1.checked_add(asset1).ok_or_else(|| {
                    StdError::overflow(OverflowError::new(OverflowOperation::Add, acc0, asset1))
                })?,
            ))
        },
    )?;

    Ok(TotalAssetsResponse {
        token0: coin(asset0, pool.token0),
        token1: coin(asset1, pool.token1),
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
