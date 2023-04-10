use crate::msg::{DistributionScheduleOptions, DistributionScheduleResponse};
use crate::VaultRewardsError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Env, Order, QuerierWrapper, Storage, Uint128};
use cw_asset::AssetInfo;
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub vault_token: Addr,
    pub reward_token: AssetInfo,
    pub distribution_schedules: Vec<DistributionSchedule>,
    pub total_claimed: Uint128,
}

impl Config {
    pub fn add_distribution_schedule(
        &mut self,
        querier: &QuerierWrapper,
        env: &Env,
        schedule: DistributionSchedule,
    ) -> Result<(), VaultRewardsError> {
        self.validate_distribution_schedule(querier, env, &schedule)?;
        self.distribution_schedules.push(schedule);
        Ok(())
    }

    pub fn add_distribution_schedules(
        &mut self,
        querier: &QuerierWrapper,
        env: &Env,
        schedules: Vec<DistributionSchedule>,
    ) -> Result<(), VaultRewardsError> {
        for schedule in schedules {
            self.add_distribution_schedule(querier, env, schedule)?;
        }
        Ok(())
    }

    pub fn update_distribution_schedule(
        &mut self,
        querier: &QuerierWrapper,
        env: &Env,
        id: u64,
        update: &DistributionScheduleOptions,
    ) -> Result<(), VaultRewardsError> {
        let cur_block_height = env.block.height;
        let idx = id.checked_sub(1).unwrap_or_default() as usize;
        let mut schedule = self
            .distribution_schedules
            .get(idx)
            .ok_or(VaultRewardsError::InvalidDistributionScheduleId {
                max_id: self.distribution_schedules.len() as u64,
            })?
            .clone();
        if schedule.start <= cur_block_height && cur_block_height < schedule.end {
            return Err(VaultRewardsError::DistributionScheduleInProgress {
                id,
                start: schedule.start,
                end: schedule.end,
            });
        }
        if cur_block_height >= schedule.end {
            return Err(VaultRewardsError::DistributionScheduleExpired {
                id,
                end: schedule.end,
            });
        }
        self.distribution_schedules.remove(idx);
        if let Some(start) = update.start {
            schedule.start = start;
        } else if let Some(end) = update.end {
            if end <= cur_block_height {
                return Err(VaultRewardsError::InvalidDistributionSchedule {
                    reason: "end must be in the future".to_string(),
                });
            }
            schedule.end = end;
        } else if let Some(amount) = update.amount {
            schedule.amount = amount;
        }
        self.validate_distribution_schedule(querier, env, &schedule)?;
        self.distribution_schedules.insert(idx, schedule);
        Ok(())
    }

    pub fn remove_distribution_schedule(
        &mut self,
        storage: &dyn Storage,
        env: &Env,
        id: u64,
    ) -> Result<(), VaultRewardsError> {
        let cur_block_height = env.block.height;
        let idx = id.checked_sub(1).unwrap_or_default() as usize;
        let schedule = self.distribution_schedules.get(idx).ok_or({
            VaultRewardsError::InvalidDistributionScheduleId {
                max_id: self.distribution_schedules.len() as u64,
            }
        })?;
        if schedule.start <= cur_block_height && cur_block_height < schedule.end {
            return Err(VaultRewardsError::DistributionScheduleInProgress {
                id,
                start: schedule.start,
                end: schedule.end,
            });
        }
        if cur_block_height >= schedule.end {
            // check if all funds from period were claimed
            USER_REWARD_INDEX
                .range(storage, None, None, Order::Ascending)
                .map(|item| {
                    let (_, reward_index) = item.unwrap();
                    if reward_index
                        .history
                        .iter()
                        .any(|h| h.start >= schedule.start && h.end < schedule.end)
                    {
                        return Err(VaultRewardsError::DistributionScheduleWithUnclaimedFunds {
                            id,
                        });
                    }
                    Ok(())
                })
                .collect::<Result<Vec<_>, _>>()?;
        }
        self.total_claimed -= schedule.amount;
        self.distribution_schedules.remove(idx);
        Ok(())
    }

    pub fn validate_distribution_schedule(
        &self,
        querier: &QuerierWrapper,
        env: &Env,
        schedule: &DistributionSchedule,
    ) -> Result<(), VaultRewardsError> {
        if schedule.start <= env.block.height {
            return Err(VaultRewardsError::InvalidDistributionSchedule {
                reason: "start must be in the future".to_string(),
            });
        }
        if schedule.start >= schedule.end {
            return Err(VaultRewardsError::InvalidDistributionSchedule {
                reason: "start must be before end".to_string(),
            });
        }
        if schedule.amount.is_zero() {
            return Err(VaultRewardsError::InvalidDistributionSchedule {
                reason: "amount must be greater than 0".to_string(),
            });
        }
        let reward_token_balance = self
            .reward_token
            .query_balance(querier, &env.contract.address)?;
        let total_distribution_amount = self.get_total_distribution_amount() + schedule.amount;
        if VALIDATE_FUNDS && reward_token_balance < total_distribution_amount {
            return Err(VaultRewardsError::InsufficientFunds {
                contract_balance: reward_token_balance,
                claim_amount: total_distribution_amount,
            });
        }
        Ok(())
    }

    pub fn get_distribution_rate_at_height(&self, height: u64) -> Uint128 {
        self.distribution_schedules
            .iter()
            .fold(Uint128::zero(), |acc, schedule| {
                if schedule.start <= height && schedule.end > height {
                    acc + schedule.amount / Uint128::from(schedule.end - schedule.start)
                } else {
                    acc
                }
            })
    }

    pub fn get_total_distribution_amount(&self) -> Uint128 {
        self.distribution_schedules
            .iter()
            .fold(Uint128::zero(), |acc, schedule| acc + schedule.amount)
    }
}

#[cw_serde]
pub struct DistributionSchedule {
    pub start: u64,
    pub end: u64,
    pub amount: Uint128,
}

impl DistributionSchedule {
    pub fn to_response(&self, idx: usize) -> DistributionScheduleResponse {
        DistributionScheduleResponse {
            id: (idx as u64) + 1,
            start: self.start,
            end: self.end,
            amount: self.amount,
        }
    }
}

#[cw_serde]
#[derive(Default)]
pub struct RewardIndex {
    pub vault_supply: Uint128,
}

#[cw_serde]
pub struct UserRewardIndex {
    pub balance: Option<UserBalance>,
    pub history: Vec<DistributionSchedule>,
}

#[cw_serde]
pub struct UserBalance {
    pub reward_index: u64,
    pub balance: Uint128,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const REWARD_INDEX: Map<u64, RewardIndex> = Map::new("reward_index");
pub const USER_REWARD_INDEX: Map<Addr, UserRewardIndex> = Map::new("user_reward_index");

// to be changed in a future migration
pub const VALIDATE_FUNDS: bool = false;
