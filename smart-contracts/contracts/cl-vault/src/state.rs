use apollo_cw_asset::{AssetInfo, AssetInfoBase};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    Addr, BlockInfo, Decimal, Decimal256, Deps, MessageInfo, Order, StdError, StdResult, Storage,
    Uint128, Coin,
};
use cw20::Expiration;
use cw_dex_router::helpers::CwDexRouterBase;
use cw_storage_plus::{Bound, Index, IndexList, IndexedMap, Item, Map, MultiIndex};
use cw_vault_standard::extensions::lockup::UnlockingPosition;
use derive_builder::Builder;
use liquidity_helper::LiquidityHelperBase;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Base config struct for the contract.
#[cw_serde]
#[derive(Builder)]
#[builder(derive(Serialize, Deserialize, Debug, PartialEq, JsonSchema))]
pub struct ConfigBase<T> {
    /// Percentage of profit to be charged as performance fee
    pub performance_fee: Decimal,
    /// Account to receive fee payments
    pub treasury: T,
    /// Router address
    pub router: CwDexRouterBase<T>,
    /// The assets that are given as liquidity mining rewards that the vault
    /// will compound into more of base_token.
    pub reward_assets: Vec<AssetInfoBase<T>>,
    /// The asset to which we should swap reward_assets into before providing
    /// liquidity. Should be one of the assets in the pool.
    pub reward_liquidation_target: AssetInfoBase<T>,
    /// Whitelisted addresses that can call ForceWithdraw and
    /// ForceWithdrawUnlocking
    pub force_withdraw_whitelist: Vec<T>,
    /// Helper for providing liquidity with unbalanced assets.
    pub liquidity_helper: LiquidityHelperBase<T>,
}

/// Config with non-validated addresses.
pub type ConfigUnchecked = ConfigBase<String>;
/// Config with validated addresses.
pub type Config = ConfigBase<Addr>;
/// Config updates struct containing same fields as Config, but all fields are
/// optional.
pub type ConfigUpdates = ConfigBaseBuilder<String>;

/// Merges the old config with a new partial config.
impl Config {
    /// Updates the existing config with the new config updates. If a field is
    /// `None` in the `updates` then the old config is kept, else it is updated
    /// to the new value.
    pub fn update(self, deps: Deps, updates: ConfigUpdates) -> StdResult<Config> {
        ConfigUnchecked {
            performance_fee: updates.performance_fee.unwrap_or(self.performance_fee),
            treasury: updates.treasury.unwrap_or_else(|| self.treasury.into()),
            router: updates.router.unwrap_or_else(|| self.router.into()),
            reward_assets: updates
                .reward_assets
                .unwrap_or_else(|| self.reward_assets.into_iter().map(Into::into).collect()),
            reward_liquidation_target: updates
                .reward_liquidation_target
                .unwrap_or_else(|| self.reward_liquidation_target.into()),
            force_withdraw_whitelist: updates.force_withdraw_whitelist.unwrap_or_else(|| {
                self.force_withdraw_whitelist
                    .into_iter()
                    .map(Into::into)
                    .collect()
            }),
            liquidity_helper: updates
                .liquidity_helper
                .unwrap_or_else(|| self.liquidity_helper.into()),
        }
        .check(deps)
    }
}

impl ConfigUnchecked {
    /// Constructs a Config from the unchecked config, validating all addresses.
    pub fn check(&self, deps: Deps) -> StdResult<Config> {
        if self.performance_fee > Decimal::one() {
            return Err(StdError::generic_err(
                "Performance fee cannot be greater than 100%",
            ));
        }

        let reward_assets: Vec<AssetInfo> = self
            .reward_assets
            .iter()
            .map(|x| x.check(deps.api))
            .collect::<StdResult<_>>()?;
        let router = self.router.check(deps.api)?;
        let reward_liquidation_target = self.reward_liquidation_target.check(deps.api)?;

        // Check that the router can route between all reward assets and the
        // reward liquidation target. We discard the actual path because we
        // don't need it here. We just need to make sure the paths exist.
        for asset in &reward_assets {
            // We skip the reward liquidation target because we don't need to
            // route to it.
            if asset == &reward_liquidation_target {
                continue;
            }
            // We map the error here because the error coming from the router is
            // not passed along into the query error, and thus we will otherwise
            // just see "Querier contract error" and no more information.
            router
                .query_path_for_pair(&deps.querier, asset, &reward_liquidation_target)
                .map_err(|_| {
                    StdError::generic_err(format!(
                        "Could not read path in cw-dex-router for {:?} -> {:?}",
                        asset, reward_liquidation_target
                    ))
                })?;
        }

        Ok(Config {
            performance_fee: self.performance_fee,
            treasury: deps.api.addr_validate(&self.treasury)?,
            reward_assets,
            reward_liquidation_target,
            router,
            force_withdraw_whitelist: self
                .force_withdraw_whitelist
                .iter()
                .map(|x| deps.api.addr_validate(x))
                .collect::<StdResult<_>>()?,
            liquidity_helper: self.liquidity_helper.check(deps.api)?,
        })
    }
}

pub const BASE_TOKEN: Item<AssetInfo> = Item::new("base_token");

// TODO: this could be done using normal hashmaps maybe?
#[cw_serde]
pub struct TickExpIndexData {
    pub initial_price: Decimal256,
    pub max_price: Decimal256,
    pub additive_increment_per_tick: Decimal256,
    pub initial_tick: i64,
}

pub const TICK_EXP_CACHE: Map<i64, TickExpIndexData> = Map::new("tick_exp_cache");

// this is the info that the contract needs to know about the pool. Saved during instantiation
#[cw_serde]
pub struct Investment {
    /// Owner created the contract and takes a cut
    pub owner: Addr,
    /// Each vault has one specific token that is used for deposits, withdrawals and accounting
    pub base_denom: String,
    /// The quote denom used by the vault (asset1)
    pub quote_denom: String,
    /// the osmosis pool id used by the vault
    pub pool_id: u64,
    /// the osmosis pool address used by the vault
    /// TODO: decide if we use Addr or String
    pub pool_address: Addr,
}

pub const INVESTMENT: Item<Investment> = Item::new("investment");

#[cw_serde]
pub struct Strategy {
    pub lower_tick: i64,
    pub upper_tick: i64,
    // slippage tolerance in basis points (1bps = 0.01%)
    pub slippage_tolerance: Uint128,
}

pub const STRATEGY: Item<Strategy> = Item::new("strategy");

pub const USER_BALANCE: Map<Addr, Uint128> = Map::new("user_balance");

pub const USER_REWARDS: Map<Addr, Vec<Coin>> = Map::new("user_rewards");