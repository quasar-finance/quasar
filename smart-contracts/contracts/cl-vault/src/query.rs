use crate::helpers::{get_one_or_two, get_unused_balances};
use crate::state::{Position, POSITIONS};
use crate::vault::concentrated_liquidity::get_positions;
use crate::{debug, ContractError};
use crate::{
    error::ContractResult,
    state::{PoolConfig, ADMIN_ADDRESS, METADATA, POOL_CONFIG, SHARES, USER_REWARDS, VAULT_DENOM},
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    coin, Coin, Deps, Env, Order, OverflowError, OverflowOperation, StdError, Uint128,
};
use cw_vault_multi_standard::VaultInfoResponse;
use osmosis_std::types::cosmos::bank::v1beta1::BankQuerier;
use osmosis_std::types::osmosis::concentratedliquidity::v1beta1::{
    ConcentratedliquidityQuerier, FullPositionBreakdown,
};

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
pub struct FullPositionsResponse {
    pub positions: Vec<FullPosition>,
}

#[cw_serde]
pub struct FullPosition {
    pub position: Position,
    pub full_breakdown: FullPositionBreakdown,
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
    debug!(deps, "positions", "");
    let positions: Result<Vec<(u64, Position)>, StdError> = POSITIONS
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    Ok(PositionResponse {
        positions: positions?.into_iter().map(|(_, p)| p).collect(),
    })
}

pub fn query_full_positions(deps: Deps) -> ContractResult<FullPositionsResponse> {
    let ps: Result<Vec<(u64, Position)>, StdError> = POSITIONS
        .range(deps.storage, None, None, Order::Ascending)
        .collect();

    debug!(deps, "here1", "");
    let positions = ps?;
    let cl_querier = ConcentratedliquidityQuerier::new(&deps.querier);

    debug!(deps, "here2", "");
    let full_positions: Result<Vec<FullPosition>, ContractError> = positions
        .into_iter()
        .map(|(id, position)| {
            let fp = cl_querier.position_by_id(id)?;

            let full_position = fp.position.unwrap();

            Ok(FullPosition {
                position,
                full_breakdown: full_position,
            })
        })
        .collect();

    debug!(deps, "here3", full_positions);
    Ok(FullPositionsResponse {
        positions: full_positions?,
    })
}

pub fn query_convert_to_assets(
    deps: Deps,
    env: Env,
    user_shares: Uint128,
) -> ContractResult<Vec<Coin>> {
    let vault_assets = query_total_assets(deps, env)?;

    if user_shares.is_zero() {
        return Ok(vec![
            coin(0, vault_assets.token0.denom),
            coin(0, vault_assets.token1.denom),
        ]);
    }

    debug!(deps, "user_shares", user_shares);
    debug!(deps, "vault_assets", vault_assets);

    let vault_shares = query_total_vault_token_supply(deps)?.total;
    debug!(deps, "vault_shares", vault_shares);

    let user_token0 = vault_assets
        .token0
        .amount
        .checked_mul(user_shares)?
        .checked_div(vault_shares)?;
    let user_token1 = vault_assets
        .token1
        .amount
        .checked_mul(user_shares)?
        .checked_div(vault_shares)?;
    Ok(vec![
        coin(user_token0.u128(), vault_assets.token0.denom),
        coin(user_token1.u128(), vault_assets.token1.denom),
    ])
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

pub fn query_total_assets(deps: Deps, env: Env) -> ContractResult<TotalAssetsResponse> {
    debug!(deps, "total_assets", "");
    let positions = get_positions(deps.storage, &deps.querier)?;
    let pool = POOL_CONFIG.load(deps.storage)?;
    let unused_balance = get_unused_balances(deps.storage, &deps.querier, &env)?;
    let (unused0, unused1) = get_one_or_two(
        &unused_balance.coins(),
        (pool.token0.clone(), pool.token1.clone()),
    )?;

    let (asset0, asset1) = positions.iter().try_fold(
        (unused0.amount.u128(), unused1.amount.u128()),
        |(acc0, acc1), i| -> Result<(u128, u128), ContractError> {
            let asset0 =
                i.1.asset0
                    .as_ref()
                    .map(|v| v.amount.parse::<u128>().unwrap())
                    .unwrap_or(0_u128);
            let asset1 =
                i.1.asset1
                    .as_ref()
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
    debug!(deps, "total_vts", "");
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
