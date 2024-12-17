use crate::helpers::coinlist::CoinList;
use crate::helpers::getters::get_unused_balances;
use crate::math::tick::verify_tick_exp_cache;
use crate::state::{
    PoolConfig, ADMIN_ADDRESS, METADATA, POOL_CONFIG, POSITION, SHARES, VAULT_CONFIG, VAULT_DENOM,
};
use crate::vault::concentrated_liquidity::get_position;
use crate::ContractError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coin, Coin, Decimal, Deps, Env, Uint128};
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
    pub users: Vec<String>,         // List of user addresses only
    pub next_token: Option<String>, // Token for the next page
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
    next_token: Option<String>,
    limit: u64,
) -> Result<ActiveUsersResponse, ContractError> {
    let mut users: Vec<String> = Vec::new();
    let mut start_index = 0;

    if let Some(token) = next_token {
        start_index = token
            .parse::<usize>()
            .map_err(|_| ContractError::InvalidToken {})?;
    }

    for result in SHARES.range(deps.storage, None, None, cosmwasm_std::Order::Ascending) {
        let (addr, _balance) = result.map_err(ContractError::Std)?;

        if start_index > 0 {
            start_index -= 1;
            continue;
        }

        users.push(addr.to_string());

        if users.len() as u64 >= limit {
            break;
        }
    }

    let next_token = if users.len() as u64 == limit {
        Some((start_index + limit as usize).to_string())
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
    fn test_query_active_users() {
        let mut deps = mock_dependencies();

        // Setup mock data in the SHARES map
        let user1 = Addr::unchecked("user1");
        let user2 = Addr::unchecked("user2");
        let user3 = Addr::unchecked("user3");

        // Insert mock user balances into SHARES
        SHARES
            .save(deps.as_mut().storage, user1, &Uint128::new(100))
            .unwrap();
        SHARES
            .save(deps.as_mut().storage, user2, &Uint128::new(200))
            .unwrap();
        SHARES
            .save(deps.as_mut().storage, user3, &Uint128::new(300))
            .unwrap();

        // Test without next_token and limit of 2
        let res = query_active_users(deps.as_ref(), None, 2).unwrap();
        assert_eq!(res.users, vec!["user1", "user2"]);
        assert_eq!(res.next_token, Some("2".to_string())); // Expecting next_token for pagination

        // Test with next_token to get the next user
        let res = query_active_users(deps.as_ref(), Some("2".to_string()), 2).unwrap();
        assert_eq!(res.users, vec!["user3"]);
        assert_eq!(res.next_token, None); // No more users, so next_token should be None
    }
}
