use crate::helpers::coinlist::CoinList;
use crate::helpers::getters::get_unused_balances;
use crate::math::tick::verify_tick_exp_cache;
use crate::state::{
    PoolConfig, ADMIN_ADDRESS, METADATA, POOL_CONFIG, POSITION, SHARES, VAULT_CONFIG, VAULT_DENOM,
};
use crate::vault::concentrated_liquidity::get_position;
use crate::ContractError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Addr, Coin, Decimal, Deps, Env, StdError, Uint128};
use cw_storage_plus::Bound;
use osmosis_std::types::cosmos::bank::v1beta1::BankQuerier;
use quasar_types::cw_vault_multi_standard::VaultInfoResponse;

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

#[cw_serde]
pub struct ActiveUsersResponse {
    pub users: Vec<(Addr, Uint128)>, // List of user addresses only
    pub next_token: Option<Addr>,    // Token for the next page
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
    let config = VAULT_CONFIG.load(deps.storage)?;

    Ok(DexRouterResponse {
        dex_router: config.dex_router.to_string(),
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

pub fn query_position(deps: Deps) -> Result<PositionResponse, ContractError> {
    let position_id = POSITION.load(deps.storage)?.position_id;
    Ok(PositionResponse {
        position_ids: vec![position_id],
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

    let assets_from_shares = vault_balance.mul_ratio(Decimal::from_ratio(shares, vault_supply))?;

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

pub fn query_active_users(
    deps: Deps,
    start_bound_exclusive: Option<Addr>,
    limit: u64,
) -> Result<ActiveUsersResponse, ContractError> {
    let start_key = start_bound_exclusive
        .clone()
        .map(|s| Bound::exclusive(Addr::unchecked(s)));

    let result_users: Result<Vec<(Addr, Uint128)>, StdError> = SHARES
        .range(
            deps.storage,
            start_key,
            None,
            cosmwasm_std::Order::Ascending,
        )
        .take(limit as usize)
        .collect();
    let users = result_users?;

    let next_token = if users.len() as u64 == limit {
        Some(users.last().unwrap().0.clone())
    } else {
        None
    };

    Ok(ActiveUsersResponse { users, next_token })
}

/// Vault base assets is the vault assets EXCLUDING any rewards claimable by strategist or users
pub fn query_total_assets(deps: Deps, env: Env) -> Result<TotalAssetsResponse, ContractError> {
    let position = get_position(deps.storage, &deps.querier)?;
    let pool = POOL_CONFIG.load(deps.storage)?;
    let unused_balance = get_unused_balances(&deps.querier, &env.contract.address)?;

    // add token0 unused balance to what's in the position
    let mut token0 = position
        .asset0
        .map(|c| c.try_into())
        .transpose()?
        .unwrap_or(coin(0, pool.token0));

    token0 = Coin {
        denom: token0.denom.clone(),
        amount: token0
            .amount
            .checked_add(unused_balance.find(&token0.denom).amount)?,
    };

    let mut token1 = position
        .asset1
        .map(|c| c.try_into())
        .transpose()?
        .unwrap_or(coin(0, pool.token1));

    token1 = Coin {
        denom: token1.denom.clone(),
        amount: token1
            .amount
            .checked_add(unused_balance.find(&token1.denom).amount)?,
    };

    Ok(TotalAssetsResponse { token0, token1 })
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

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{testing::mock_dependencies, Addr, Uint128};

    #[test]
    fn test_query_active_users_with_conditions() {
        let mut deps = mock_dependencies();

        let users: Vec<Addr> = (1..10)
            .map(|i| Addr::unchecked(format!("user{}", i)))
            .collect();

        for (i, user) in users.iter().enumerate() {
            SHARES
                .save(
                    deps.as_mut().storage,
                    user.clone(),
                    &Uint128::new(((i + 1) * 100) as u128),
                ) // Assigning balances
                .unwrap();
        }

        let res: ActiveUsersResponse = query_active_users(deps.as_ref(), None, 5).unwrap();
        assert_eq!(res.users.len(), 5);
        assert_eq!(res.users[0].0, "user1");
        assert_eq!(res.users[1].0, "user2");
        assert_eq!(res.users[2].0, "user3");
        assert_eq!(res.users[3].0, "user4");
        assert_eq!(res.users[4].0, "user5");
        assert_eq!(res.next_token, Some(Addr::unchecked("user5"))); // Next token should indicate the next start index

        let res: ActiveUsersResponse =
            query_active_users(deps.as_ref(), res.next_token, 5).unwrap();
        assert_eq!(res.users.len(), 4);
        assert_eq!(res.users[0].0, "user6");
        assert_eq!(res.users[1].0, "user7");
        assert_eq!(res.users[2].0, "user8");
        assert_eq!(res.users[3].0, "user9");
        assert_eq!(res.next_token, None); // No more users, so next_token should be None

        let res: ActiveUsersResponse =
            query_active_users(deps.as_ref(), Some(Addr::unchecked("user3")), 2).unwrap();
        assert_eq!(res.users.len(), 2);
        assert_eq!(res.users[0].0, "user4");
        assert_eq!(res.users[1].0, "user5");
        assert_eq!(res.next_token, Some(Addr::unchecked("user5"))); // Still more users, so next_token should be user 5
    }
}
