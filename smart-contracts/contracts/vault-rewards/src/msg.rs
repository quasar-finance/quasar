use crate::state::DistributionSchedule;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use cw_asset::{Asset, AssetInfo};

#[cw_serde]
pub struct InstantiateMsg {
    pub vault_token: String,
    pub reward_token: AssetInfo,
    pub distribution_schedule: DistributionSchedule,
}

#[cw_serde]
pub enum ExecuteMsg {
    Claim {},

    Admin(AdminExecuteMsg),

    Vault(VaultExecuteMsg),
}

#[cw_serde]
pub enum AdminExecuteMsg {
    WithdrawFunds(Asset),
    AddDistributionSchedule(DistributionSchedule),
    UpdateDistributionSchedule {
        id: u64,
        update: DistributionScheduleOptions,
    },
    RemoveDistributionSchedule(u64),
}

#[cw_serde]
pub enum VaultExecuteMsg {
    UpdateUserRewardIndex(String),
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    PendingRewards(String),
    GetUserRewardsIndex(String),
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ConfigResponse {
    pub reward_token: AssetInfo,
    pub contract_balance: Uint128,
    pub total_claimed: Uint128,
    pub distribution_schedules: Vec<DistributionScheduleResponse>,
    pub current_distribution_rate_per_block: Uint128,
}

#[cw_serde]
pub struct DistributionScheduleResponse {
    pub id: u64,
    pub start: u64,
    pub end: u64,
    pub amount: Uint128,
}

#[cw_serde]
pub struct DistributionScheduleOptions {
    pub start: Option<u64>,
    pub end: Option<u64>,
    pub amount: Option<Uint128>,
}
