use apollo_cw_asset::{AssetInfo, AssetInfoBase};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{
    Addr, BlockInfo, Decimal, Deps, MessageInfo, Order, StdError, StdResult, Storage, Uint128,
};
use cw20::Expiration;
use cw_dex_router::helpers::CwDexRouterBase;
use cw_storage_plus::{Bound, Index, IndexList, IndexedMap, Item, MultiIndex};
use cw_vault_standard::extensions::lockup::UnlockingPosition;
use derive_builder::Builder;
use liquidity_helper::LiquidityHelperBase;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::base_vault::BaseVault;
use cw_controllers::Admin;

pub const VAULT: Item<Vault> = Item::new("vault");

pub struct Vault<'a, S, P, V> {
    /// The base vault implementation
    pub base_vault: BaseVault<'a, V>,

    /// The pool that this vault compounds.
    pub pool: Item<'a, P>,

    /// The staking implementation for this vault
    pub staking: Item<'a, S>,

    /// Configuration for this vault
    pub config: Item<'a, Config>,

    /// The admin address that is allowed to update the config.
    pub admin: Admin<'a>,

    /// Temporary storage of an address that will become the new admin once
    /// they accept the transfer request.
    pub admin_transfer: Item<'a, Addr>,

    /// Stores claims of base_tokens for users who have burned their vault
    /// tokens via ExecuteMsg::Unlock.
    pub claims: Claims<'a>,
}

/// An unlockin position for a user that can be claimed once it has matured.
pub type Claim = UnlockingPosition;
/// A struct for handling the addition and removal of claims, as well as
/// querying and force unlocking of claims.
pub struct Claims<'a> {
    /// All currently unclaimed claims, both unlocking and matured. Once a claim
    /// is claimed by its owner after it has matured, it is removed from this
    /// map.
    claims: IndexedMap<'a, u64, Claim, ClaimIndexes<'a>>,
    /// The pending claim that is currently being created. When the claim is
    /// ready to be saved to the `claims` map [`self.commit_pending_claim()`]
    /// should be called.
    pending_claim: Item<'a, Claim>,
    // Counter of the number of claims. Used as a default value for the ID of a new
    // claim if the underlying staking contract doesn't issue their own IDs. This is monotonically
    // increasing and is not decremented when a claim is removed. It represents the number of
    // claims that have been created since creation of the `Claims` instance.
    next_claim_id: Item<'a, u64>,
}

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
