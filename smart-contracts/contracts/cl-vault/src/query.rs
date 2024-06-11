use crate::helpers::get_unused_balances;
use crate::math::tick::verify_tick_exp_cache;
use crate::rewards::CoinList;
use crate::state::{PoolConfig, ADMIN_ADDRESS, METADATA, POOL_CONFIG, SHARES, VAULT_DENOM};
use crate::state::{Position, DEX_ROUTER, POSITIONS};
use crate::vault::concentrated_liquidity::get_position;
use crate::ContractError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Coin, Decimal, Deps, Env, Order, StdError, Uint128};
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
pub struct PositionsResponse {
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
pub struct AssetsBalanceResponse {
    pub balances: Vec<Coin>,
}

#[cw_serde]
pub struct UserSharesBalanceResponse {
    pub balance: Uint128,
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

pub fn query_verify_tick_cache(deps: Deps) -> Result<VerifyTickCacheResponse, ContractError> {
    verify_tick_exp_cache(deps.storage)
        .err()
        .map(|e| {
            if let ContractError::TickNotFound { tick } = e {
                Ok(VerifyTickCacheResponse { result: Err(tick) })
            } else {
                Err(e)
            }
        })
        .unwrap_or(Ok(VerifyTickCacheResponse { result: Ok(()) }))
}

pub fn query_metadata(deps: Deps) -> Result<MetadataResponse, ContractError> {
    let metadata = METADATA.load(deps.storage)?;
    let vault_denom = VAULT_DENOM.load(deps.storage)?;
    // Retrieve the total supply and parse it into the required type (u128).
    let total_supply = query_total_vault_token_supply(deps)?.total;

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

pub fn query_dex_router(deps: Deps) -> Result<DexRouterResponse, ContractError> {
    let dex_router = DEX_ROUTER.load(deps.storage)?;

    Ok(DexRouterResponse {
        dex_router: dex_router.to_string(),
    })
}

pub fn query_info(deps: Deps) -> Result<VaultInfoResponse, ContractError> {
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    let vault_token = VAULT_DENOM.load(deps.storage)?;
    Ok(VaultInfoResponse {
        tokens: vec![pool_config.token0, pool_config.token1],
        vault_token,
    })
}

pub fn query_pool(deps: Deps) -> Result<PoolResponse, ContractError> {
    let pool_config = POOL_CONFIG.load(deps.storage)?;
    Ok(PoolResponse { pool_config })
}

pub fn query_positions(deps: Deps) -> Result<PositionsResponse, ContractError> {
    let positions: Result<Vec<(u64, Position)>, StdError> = POSITIONS
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    Ok(PositionsResponse {
        positions: positions?.into_iter().map(|(_, p)| p).collect(),
    })
}

pub fn query_full_positions(deps: Deps) -> Result<FullPositionsResponse, ContractError> {
    let ps: Result<Vec<(u64, Position)>, StdError> = POSITIONS
        .range(deps.storage, None, None, Order::Ascending)
        .collect();

    let positions = ps?;
    let cl_querier = ConcentratedliquidityQuerier::new(&deps.querier);

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

    Ok(FullPositionsResponse {
        positions: full_positions?,
    })
}

pub fn query_assets_from_shares(
    deps: Deps,
    env: Env,
    shares: Uint128,
) -> Result<AssetsBalanceResponse, ContractError> {
    let vault_supply = query_total_vault_token_supply(deps)?.total;
    let vault_assets = query_total_assets(deps, env)?;

    let vault_balance = CoinList::from_coins(vec![vault_assets.token0, vault_assets.token1]);

    let assets_from_shares = vault_balance.mul_ratio(Decimal::from_ratio(shares, vault_supply));

    Ok(AssetsBalanceResponse {
        balances: assets_from_shares.coins(),
    })
}

/// User assets is the users assets EXCLUDING any rewards claimable by that user
pub fn query_user_assets(
    deps: Deps,
    env: Env,
    user: String,
) -> Result<AssetsBalanceResponse, ContractError> {
    let user_shares = query_user_balance(deps, user)?.balance;
    let user_assets = query_assets_from_shares(deps, env, user_shares)?;

    Ok(user_assets)
}

pub fn query_user_balance(
    deps: Deps,
    user: String,
) -> Result<UserSharesBalanceResponse, ContractError> {
    let balance = SHARES
        .may_load(deps.storage, deps.api.addr_validate(&user)?)?
        .unwrap_or(Uint128::zero());
    Ok(UserSharesBalanceResponse { balance })
}

/// query_total_assets returns all assets currently in positions and all
pub fn query_total_assets(deps: Deps, env: Env) -> Result<TotalAssetsResponse, ContractError> {
    let positions = query_full_positions(deps)?;
    let pool = POOL_CONFIG.load(deps.storage)?;
    let unused_balance = get_unused_balances(&deps.querier, &env)?;

    // TODO would be nice to remove the unwraps here, although the unwraps are not awful since something is clearly
    // terribly wrong if Osmosis returns non uints in the amounts field
    let (amount0, amount1) =
        positions
            .positions
            .iter()
            .fold((Uint128::zero(), Uint128::zero()), |(acc0, acc1), fp| {
                (
                    acc0 + fp
                        .full_breakdown
                        .asset0
                        .map(|c| c.amount.parse().unwrap())
                        .unwrap_or(Uint128::zero()),
                    acc1 + fp
                        .full_breakdown
                        .asset1
                        .map(|c| c.amount.parse().unwrap())
                        .unwrap_or(Uint128::zero()),
                )
            });

    let free0 = deps
        .querier
        .query_balance(env.contract.address, pool.token0)?;
    let free1 = deps
        .querier
        .query_balance(env.contract.address, pool.token1)?;

    Ok(TotalAssetsResponse {
        token0: coin((amount0 + free0.amount).u128(), free0.denom),
        token1: coin((amount0 + free0.amount).u128(), free1.denom),
    })
}

pub fn query_total_vault_token_supply(
    deps: Deps,
) -> Result<TotalVaultTokenSupplyResponse, ContractError> {
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
