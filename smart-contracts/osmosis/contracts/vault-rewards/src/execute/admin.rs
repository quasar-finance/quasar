use crate::msg::DistributionScheduleOptions;
use crate::state::{DistributionSchedule, CONFIG};
use crate::VaultRewardsError;
use cosmwasm_std::{attr, Addr, DepsMut, Env, Response, Uint128};
use cw_asset::Asset;

pub fn execute_withdraw_funds(
    deps: DepsMut,
    env: Env,
    admin: Addr,
    mut asset: Asset,
) -> Result<Response, VaultRewardsError> {
    let config = CONFIG.load(deps.storage)?;

    let reward_token = &config.reward_token;

    if &asset.info == reward_token {
        // check if reward balance is sufficient after withdrawal
        let contract_reward_balance = config
            .reward_token
            .query_balance(&deps.querier, &env.contract.address)?
            .checked_sub(asset.amount)
            .unwrap_or_default();
        let total_reward_needed_to_distribute = config.get_total_distribution_amount();
        if contract_reward_balance < total_reward_needed_to_distribute {
            return Err(VaultRewardsError::InsufficientFunds {
                contract_balance: contract_reward_balance,
                claim_amount: total_reward_needed_to_distribute,
            });
        }
    } else {
        // send to admin if balance > 0
        asset.amount = asset.amount.min(
            asset
                .info
                .query_balance(&deps.querier, &env.contract.address)?,
        );
    };

    if asset.amount.is_zero() {
        return Err(VaultRewardsError::InsufficientFunds {
            contract_balance: Uint128::zero(),
            claim_amount: Uint128::zero(),
        });
    }
    let transfer = asset.transfer_msg(&admin)?;

    Ok(Response::new().add_message(transfer).add_attributes(vec![
        ("action", "withdraw_funds"),
        ("asset", &asset.to_string()),
        ("admin", admin.as_ref()),
    ]))
}

pub fn execute_add_distribution_schedule(
    deps: DepsMut,
    env: Env,
    schedule: DistributionSchedule,
) -> Result<Response, VaultRewardsError> {
    let mut config = CONFIG.load(deps.storage)?;
    config.add_distribution_schedule(&deps.querier, &env, schedule.clone())?;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::default().add_attributes(vec![
        ("action", "add_distribution_schedule"),
        ("schedule start", &schedule.start.to_string()),
        ("schedule end", &schedule.end.to_string()),
        ("schedule amount", &schedule.amount.to_string()),
    ]))
}

pub fn execute_update_distribution_schedule(
    deps: DepsMut,
    env: Env,
    id: u64,
    update: DistributionScheduleOptions,
) -> Result<Response, VaultRewardsError> {
    let mut config = CONFIG.load(deps.storage)?;
    config.update_distribution_schedule(&deps.querier, &env, id, &update)?;
    CONFIG.save(deps.storage, &config)?;
    let mut attrs = vec![
        attr("action", "update_distribution_schedule"),
        attr("id", id.to_string()),
    ];
    if let Some(start) = &update.start {
        attrs.push(attr("schedule_start_updated_to", start.to_string()));
    }
    if let Some(end) = &update.end {
        attrs.push(attr("schedule_end_updated_to", end.to_string()));
    }
    if let Some(amount) = &update.amount {
        attrs.push(attr("schedule_amount_updated_to", amount.to_string()));
    }
    Ok(Response::default().add_attributes(attrs))
}

pub fn execute_remove_distribution_schedule(
    deps: DepsMut,
    env: Env,
    id: u64,
) -> Result<Response, VaultRewardsError> {
    let mut config = CONFIG.load(deps.storage)?;
    config.remove_distribution_schedule(deps.storage, &env, id)?;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::default().add_attributes(vec![
        ("action", "remove_distribution_schedule"),
        ("id", &id.to_string()),
    ]))
}

#[cfg(test)]
mod tests {}
